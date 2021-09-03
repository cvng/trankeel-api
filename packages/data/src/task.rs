use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
    Completed,
    Failed,
    Pending,
    Running,
}
