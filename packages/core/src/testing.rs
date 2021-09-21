#[allow(dead_code)]
pub fn db() -> InMemoryDb {
    InMemoryDb::new()
}

#[derive(Default)]
pub struct InMemoryDb;

impl InMemoryDb {
    pub fn new() -> Self {
        Self
    }
}
