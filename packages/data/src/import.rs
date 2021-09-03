use async_graphql::Enum;

// # Types

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImportSource {
    Rentila,
}
