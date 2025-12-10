use rbatis::crud;
use serde::{ Deserialize, Serialize };
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Option<i64>,
    pub name: String,
    pub age: i64,
}
crud!(UserActivity {}, "users");
