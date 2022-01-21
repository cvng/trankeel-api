use crate::Amount;
use crate::Id;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;

// # Types

pub type PlanId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Plancode"]
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
