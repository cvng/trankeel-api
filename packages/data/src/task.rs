use crate::id;
use async_graphql::Enum;
use fake::Fake;

// # Types

id!(TaskId);

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum TaskStatus {
    Completed,
    Failed,
    Pending,
    Running,
}

pub struct Task {
    pub id: TaskId,
    pub status: String,
    pub progress: u32,
}
