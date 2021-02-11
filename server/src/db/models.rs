use uuid::Uuid;

use super::schema::cards;
use super::schema::users;

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
}

#[derive(Queryable, Insertable)]
pub struct Card {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String,
    pub display_name: String,
}
