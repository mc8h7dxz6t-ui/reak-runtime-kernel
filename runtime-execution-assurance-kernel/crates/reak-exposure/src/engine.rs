use crate::error::ExposureError;
use crate::model::{CeilingBreachRecord, ExposureReservation, ReservationToken};
use parking_lot::RwLock;
use reak_authority::AuthorityService;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind,
};
use reak_types::{StreamId, TenantId};
use std::collections::HashMap;

const EXP_STREAM: u64 = 0x0045_5850;

pub struct ExposureService {
    tenant_id: TenantId,
    authority: AuthorityService,
    store: MemoryDurableRecordStore,
    default_ceiling: u64,
    ceiling_per_principal: RwLock<HashMap<String, u64>>,
    reserved: RwLock<HashMap<String, u64>>,
}

impl ExposureService {
    pub fn new(tenant_id: TenantId, authority: AuthorityService, default_ceiling: u64) -> Self {
        Self {
            tenant_id,
            authority,
            store: MemoryDurableRecordStore::new(),
            default_ceiling,
            ceiling_per_principal: RwLock::new(HashMap::new()),
            reserved: RwLock::new(HashMap::new()),
        }
    }

    /// Shared authority service used for reservation scope checks (IF-EXP-01 composition).
    pub fn authority(&self) -> &AuthorityService {
        &self.authority
    }

    pub fn set_ceiling(&self, principal_id: &str, ceiling: u64) {
        self.ceiling_per_principal
            .write()
            .insert(principal_id.to_string(), ceiling);
    }

    fn ceiling_for(&self, principal_id: &str) -> u64 {
        self.ceiling_per_principal
            .read()
            .get(principal_id)
            .copied()
            .unwrap_or(self.default_ceiling)
    }

    pub fn reserve(
        &self,
        reservation: ExposureReservation,
    ) -> Result<ReservationToken, ExposureError> {
        if reservation.units == 0 {
            return Err(ExposureError::InvalidAmount);
        }
        self.authority
            .verify_scope(&reservation.principal_id, &reservation.scope)?;

        let ceiling = self.ceiling_for(&reservation.principal_id);
        let mut reserved = self.reserved.write();
        let current = reserved.get(&reservation.principal_id).copied().unwrap_or(0);
        if current + reservation.units > ceiling {
            let breach = CeilingBreachRecord {
                principal_id: reservation.principal_id.clone(),
                attempted_units: reservation.units,
                ceiling,
            };
            let payload =
                serde_json::to_vec(&breach).map_err(|_| ExposureError::Serialization)?;
            self.store.append(RecordEnvelope {
                tenant_id: self.tenant_id.clone(),
                stream_id: StreamId::new(EXP_STREAM),
                schema_version: 1,
                record_kind: RecordKind::Data,
                payload,
            })?;
            return Err(ExposureError::CeilingBreached);
        }
        reserved.insert(reservation.principal_id.clone(), current + reservation.units);

        let token = ReservationToken {
            reservation_id: format!("{}:{}", reservation.principal_id, current + reservation.units),
            units_reserved: reservation.units,
        };
        let payload = serde_json::to_vec(&token).map_err(|_| ExposureError::Serialization)?;
        self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(EXP_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        Ok(token)
    }

    pub fn release(&self, principal_id: &str, units: u64) -> Result<(), ExposureError> {
        if units == 0 {
            return Err(ExposureError::InvalidAmount);
        }
        let mut reserved = self.reserved.write();
        let current = reserved.get(principal_id).copied().unwrap_or(0);
        if units > current {
            return Err(ExposureError::ReservationNotFound);
        }
        reserved.insert(principal_id.to_string(), current - units);
        Ok(())
    }

    pub fn derate(&self, principal_id: &str, new_ceiling: u64) {
        self.set_ceiling(principal_id, new_ceiling);
        let mut reserved = self.reserved.write();
        if let Some(current) = reserved.get(principal_id) {
            if *current > new_ceiling {
                reserved.insert(principal_id.to_string(), new_ceiling);
            }
        }
    }
}
