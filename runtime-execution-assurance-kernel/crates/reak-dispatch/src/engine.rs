use crate::error::DispatchError;
use crate::model::{
    DispatchAttemptRecord, DispatchHistoryEntry, DispatchOutcome, DispatchOutcomeRecord,
    DispatchReplayMetadata, DispatchSpec, DispatchState, DispatchTicket,
};
use crate::state_machine;
use crate::verify::{VerificationContext, verify_pre_dispatch};
use parking_lot::{Mutex, RwLock};
use reak_commitment::{CommitmentEngine, CommitmentRecord};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_exposure::ExposureService;
use reak_policy_context::PolicyContext;
use reak_replay::ReplayEngine;
use reak_types::{RecordHash, RecordSequence, StreamId, TenantId};
use reak_ues::UncertaintyLedger;
use std::collections::HashMap;

const DSP_STREAM: u64 = 0x0044_5350;

pub struct DispatchEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    tickets: RwLock<HashMap<String, DispatchTicket>>,
    terminal: RwLock<HashMap<String, DispatchOutcomeRecord>>,
    by_commitment: RwLock<HashMap<String, String>>,
    by_operation: RwLock<HashMap<String, ()>>,
    dispatch_serial: Mutex<()>,
}

impl DispatchEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            tickets: RwLock::new(HashMap::new()),
            terminal: RwLock::new(HashMap::new()),
            by_commitment: RwLock::new(HashMap::new()),
            by_operation: RwLock::new(HashMap::new()),
            dispatch_serial: Mutex::new(()),
        }
    }

    fn operation_key(record: &CommitmentRecord) -> String {
        format!("{}:{}", record.operation.lineage_id, record.operation.generation)
    }

    fn ticket_id(commitment_id: &str, attempt: u32) -> String {
        format!("dsp_{}_{}", commitment_id, attempt)
    }

    fn append_entry(&self, entry: &DispatchHistoryEntry) -> Result<DispatchReplayMetadata, DispatchError> {
        let payload =
            serde_json::to_vec(entry).map_err(|_| DispatchError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(DSP_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(DSP_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(DispatchError::ReplayInconsistent)?;
        Ok(DispatchReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    /// IF-DSP-01: issue_ticket — verify commitment and all bindings; no external effect.
    pub fn issue_ticket(
        &self,
        commitment_engine: &CommitmentEngine,
        exposure: &ExposureService,
        policy: &PolicyContext,
        ues: &UncertaintyLedger,
        spec: DispatchSpec,
        issued_at_unix_ms: u64,
    ) -> Result<DispatchTicket, DispatchError> {
        let _guard = self.dispatch_serial.lock();
        let ctx = VerificationContext {
            commitment_engine,
            exposure,
            policy,
            ues,
        };
        let record = verify_pre_dispatch(&ctx, &spec.commitment_id, &spec.commitment_digest)?;

        if self.by_commitment.read().contains_key(&spec.commitment_id) {
            return Err(DispatchError::DuplicateDispatch);
        }
        let op_key = Self::operation_key(&record);
        if self.by_operation.read().contains_key(&op_key) {
            return Err(DispatchError::DuplicateDispatch);
        }

        let attempt_number = 1;
        let ticket_id = Self::ticket_id(&spec.commitment_id, attempt_number);
        let ticket = DispatchTicket {
            ticket_id: ticket_id.clone(),
            commitment_id: spec.commitment_id.clone(),
            attempt_number,
            operation_lineage: record.operation.lineage_id.clone(),
            operation_generation: record.operation.generation,
            state: DispatchState::TicketIssued,
            issued_at_unix_ms,
        };

        let replay = self.append_entry(&DispatchHistoryEntry::TicketIssued(ticket.clone()))?;
        let _ = replay;

        self.tickets
            .write()
            .insert(ticket_id.clone(), ticket.clone());
        self.by_commitment
            .write()
            .insert(spec.commitment_id.clone(), ticket_id);
        self.by_operation.write().insert(op_key, ());
        Ok(ticket)
    }

    /// IF-DSP-01: execute_once — exactly one outbound release per ticket (no provider I/O).
    pub fn execute_once(
        &self,
        ticket_id: &str,
        released_at_unix_ms: u64,
    ) -> Result<DispatchAttemptRecord, DispatchError> {
        let _guard = self.dispatch_serial.lock();
        let mut tickets = self.tickets.write();
        let ticket = tickets.get(ticket_id).cloned().ok_or(DispatchError::TicketNotFound)?;

        if self.terminal.read().contains_key(ticket_id) {
            return Err(DispatchError::AlreadyTerminal);
        }

        if ticket.state != DispatchState::TicketIssued {
            return Err(DispatchError::IllegalTransition);
        }

        if !state_machine::can_transition(DispatchState::TicketIssued, DispatchState::Released) {
            return Err(DispatchError::IllegalTransition);
        }

        let attempt = DispatchAttemptRecord {
            ticket_id: ticket_id.to_string(),
            attempt_number: ticket.attempt_number,
            released_at_unix_ms,
            replay: DispatchReplayMetadata {
                durable_sequence: RecordSequence::ZERO,
                prev_chain_hash: RecordHash::ZERO,
                record_chain_hash: RecordHash::ZERO,
            },
        };
        let replay = self.append_entry(&DispatchHistoryEntry::Released(attempt.clone()))?;
        let attempt = DispatchAttemptRecord { replay, ..attempt };

        let updated = DispatchTicket {
            state: DispatchState::AwaitingAck,
            ..ticket
        };
        tickets.insert(ticket_id.to_string(), updated);

        Ok(attempt)
    }

    /// IF-DSP-01: ack_or_nack — immutable terminal outcome; UNKNOWN never re-dispatches.
    pub fn ack_or_nack(
        &self,
        ticket_id: &str,
        outcome: DispatchOutcome,
        classified_at_unix_ms: u64,
    ) -> Result<DispatchOutcomeRecord, DispatchError> {
        let _guard = self.dispatch_serial.lock();
        let ticket = self
            .tickets
            .read()
            .get(ticket_id)
            .cloned()
            .ok_or(DispatchError::TicketNotFound)?;

        if self.terminal.read().contains_key(ticket_id) {
            return Err(DispatchError::AlreadyTerminal);
        }

        if ticket.state != DispatchState::AwaitingAck {
            return Err(DispatchError::IllegalTransition);
        }

        if !state_machine::is_terminal_outcome(outcome) {
            return Err(DispatchError::IllegalTransition);
        }

        let outcome_record = DispatchOutcomeRecord {
            ticket_id: ticket_id.to_string(),
            outcome,
            classified_at_unix_ms,
            replay: DispatchReplayMetadata {
                durable_sequence: RecordSequence::ZERO,
                prev_chain_hash: RecordHash::ZERO,
                record_chain_hash: RecordHash::ZERO,
            },
            recovery_metadata: None,
            cancellation_reason: if outcome == DispatchOutcome::Cancelled {
                Some(1)
            } else {
                None
            },
        };
        let replay = self.append_entry(&DispatchHistoryEntry::Outcome(outcome_record.clone()))?;
        let outcome_record = DispatchOutcomeRecord { replay, ..outcome_record };

        self.terminal
            .write()
            .insert(ticket_id.to_string(), outcome_record.clone());

        ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(DSP_STREAM))
            .map_err(|_| DispatchError::ReplayInconsistent)?;

        Ok(outcome_record)
    }

    pub fn verify_no_second_release(&self, ticket_id: &str) -> Result<(), DispatchError> {
        let ticket = self
            .tickets
            .read()
            .get(ticket_id)
            .cloned()
            .ok_or(DispatchError::TicketNotFound)?;
        if ticket.state == DispatchState::TicketIssued {
            return Ok(());
        }
        if self.terminal.read().contains_key(ticket_id) {
            return Ok(());
        }
        Err(DispatchError::ConcurrentDispatch)
    }

    pub fn recover_from_log(&self) -> Result<usize, DispatchError> {
        let report = ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(DSP_STREAM))
            .map_err(|_| DispatchError::ReplayInconsistent)?;

        let mut tickets = HashMap::new();
        let mut terminal = HashMap::new();
        let mut by_commitment = HashMap::new();
        let mut by_operation = HashMap::new();
        let mut count = 0;

        for stored in &report.snapshot {
            let entry: DispatchHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| DispatchError::Serialization)?;
            match entry {
                DispatchHistoryEntry::TicketIssued(t) => {
                    let tid = t.ticket_id.clone();
                    by_commitment.insert(t.commitment_id.clone(), tid.clone());
                    by_operation.insert(
                        format!("{}:{}", t.operation_lineage, t.operation_generation),
                        (),
                    );
                    tickets.insert(tid, t);
                }
                DispatchHistoryEntry::Released(r) => {
                    if let Some(t) = tickets.get_mut(&r.ticket_id) {
                        t.state = DispatchState::AwaitingAck;
                    }
                }
                DispatchHistoryEntry::Outcome(o) => {
                    terminal.insert(o.ticket_id.clone(), o);
                }
            }
            count += 1;
        }

        *self.tickets.write() = tickets;
        *self.terminal.write() = terminal;
        *self.by_commitment.write() = by_commitment;
        *self.by_operation.write() = by_operation;
        Ok(count)
    }
}
