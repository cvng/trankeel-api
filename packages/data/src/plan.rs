use crate::common::Id;
use crate::Amount;
use async_graphql::Enum;

// # Types

pub type PlanId = Id;

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PlanCode {
    Solo,
}

#[derive(Queryable)]
pub struct Plan {
    pub id: PlanId,
    pub code: String,
    pub price: Option<Amount>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}
