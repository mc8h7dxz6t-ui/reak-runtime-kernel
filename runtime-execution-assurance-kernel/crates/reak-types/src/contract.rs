/// Versioned interface identifiers (Phase 1 IF-* contracts).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterfaceId {
    pub id: &'static str,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl InterfaceId {
    pub const fn v1_0_0(id: &'static str) -> Self {
        Self {
            id,
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
}

pub const INTERFACE_DR_01: InterfaceId = InterfaceId::v1_0_0("IF-DR-01");
pub const INTERFACE_POL_01: InterfaceId = InterfaceId::v1_0_0("IF-POL-01");
pub const INTERFACE_UES_01: InterfaceId = InterfaceId::v1_0_0("IF-UES-01");
pub const INTERFACE_REG_01: InterfaceId = InterfaceId::v1_0_0("IF-REG-01");
pub const INTERFACE_RPL_01: InterfaceId = InterfaceId::v1_0_0("IF-RPL-01");
pub const INTERFACE_AUTH_01: InterfaceId = InterfaceId::v1_0_0("IF-AUTH-01");
pub const INTERFACE_EXP_01: InterfaceId = InterfaceId::v1_0_0("IF-EXP-01");
