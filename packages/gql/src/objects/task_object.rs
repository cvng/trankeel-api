use piteo::TaskId;

#[derive(SimpleObject)]
pub struct Task {
    pub id: TaskId,
    pub status: String,
    pub progress: u32,
}
