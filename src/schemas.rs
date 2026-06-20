use std::collections::HashMap;
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: String,
    pub operations_by_users: HashMap<i32, Vec<Operations>>,
    pub role: String,
}

pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: String,
    pub operations_on_user: HashMap<i32, Vec<Operations>>,
}

pub enum OperationType {
    Like,
    Unlike,
    View,
    Comment,
    Share,
}
pub struct Operations {
    pub id: i32,
    pub operation_type: OperationType,

    pub description: Option<String>,
    pub created_at: String,
}
