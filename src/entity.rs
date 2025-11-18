use rbatis::crud;
use serde::{ Deserialize, Serialize };
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
}
crud!(UserActivity {});
