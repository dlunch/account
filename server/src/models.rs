use uuid::Uuid;

use super::schema::users;

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
}
