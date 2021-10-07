use crate::Amount;
use crate::Id;

// # Types

pub type PlanId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum PlanCode {
    Solo,
}

#[derive(Queryable)]
pub struct Plan {
    pub id: PlanId,
    pub code: PlanCode,
    pub price: Option<Amount>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}
