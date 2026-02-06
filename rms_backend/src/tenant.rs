use crate::user::fields::Id;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Tenant {
    id: Id,
    payment_status: bool,
}

impl Tenant {
    pub fn new(id: Uuid, payment_status: bool) -> Self {
        Self {
            id: Id::from(id),
            payment_status,
        }
    }
}

