use crate::Id;

// # Types

pub type TaskId = Id;

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
