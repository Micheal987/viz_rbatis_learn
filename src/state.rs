use rbatis::RBatis;

#[derive(Clone)]
pub struct AppState {
    pub db: RBatis,
}
impl AppState {
    pub fn new(db: RBatis) -> Self {
        Self { db }
    }
    pub fn db(&self) -> &RBatis {
        &self.db
    }
}
