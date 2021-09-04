use async_graphql::Enum;

// # Types

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum TaskStatus {
    Completed,
    Failed,
    Pending,
    Running,
}
