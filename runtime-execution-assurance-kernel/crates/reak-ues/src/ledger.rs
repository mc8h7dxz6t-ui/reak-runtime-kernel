use crate::error::UesError;
use crate::model::{BudgetReceipt, StageBudget, UncertaintyDelta, ViolationRecord};
use parking_lot::RwLock;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind,
};
use reak_types::{StageId, StreamId, TenantId};
use std::collections::HashMap;

const UES_STREAM: u64 = 0x0055_4553;

pub struct UncertaintyLedger {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    remaining: RwLock<HashMap<u32, u64>>,
}

impl UncertaintyLedger {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            remaining: RwLock::new(HashMap::new()),
        }
    }

    pub fn declare_budget(&self, budget: StageBudget) -> Result<(), UesError> {
        if budget.max_eliminable_units == 0 {
            return Err(UesError::InvalidDelta);
        }
        self.remaining
            .write()
            .insert(budget.stage.raw(), budget.max_eliminable_units);
        let payload =
            serde_json::to_vec(&budget).map_err(|_| UesError::Serialization)?;
        self.store
            .append(RecordEnvelope {
                tenant_id: self.tenant_id.clone(),
                stream_id: StreamId::new(UES_STREAM),
                schema_version: 1,
                record_kind: RecordKind::Data,
                payload,
            })
            .map_err(UesError::from)?;
        Ok(())
    }

    pub fn consume_budget(
        &self,
        stage: StageId,
        delta: UncertaintyDelta,
    ) -> Result<BudgetReceipt, UesError> {
        if delta.eliminable_units == 0 {
            return Err(UesError::InvalidDelta);
        }
        let mut map = self.remaining.write();
        let rem = map.get_mut(&stage.raw()).ok_or(UesError::BudgetNotDeclared)?;
        if delta.eliminable_units > *rem {
            let violation = ViolationRecord {
                stage,
                attempted: delta.eliminable_units,
                remaining: *rem,
            };
            self.append_violation(&violation)?;
            return Err(UesError::BudgetExhausted);
        }
        *rem -= delta.eliminable_units;
        Ok(BudgetReceipt {
            stage,
            remaining: *rem,
        })
    }

    pub fn reject_violation(&self, violation: ViolationRecord) -> Result<(), UesError> {
        self.append_violation(&violation)?;
        Err(UesError::BudgetExhausted)
    }

    fn append_violation(&self, violation: &ViolationRecord) -> Result<(), UesError> {
        let payload =
            serde_json::to_vec(violation).map_err(|_| UesError::Serialization)?;
        self.store
            .append(RecordEnvelope {
                tenant_id: self.tenant_id.clone(),
                stream_id: StreamId::new(UES_STREAM),
                schema_version: 1,
                record_kind: RecordKind::Data,
                payload,
            })
            .map_err(UesError::from)?;
        Ok(())
    }
}
