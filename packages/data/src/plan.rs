use crate::Amount;
use crate::Id;
use async_graphql::Enum;

// # Types

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PlanCode {
    Solo,
}

#[derive(Queryable)]
pub struct Plan {
    pub code: String,
    pub price: Option<Amount>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    pub id: Id,
}
