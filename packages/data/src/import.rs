use async_graphql::Enum;

// # Types

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum ImportSource {
    Rentila,
}
