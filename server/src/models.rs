use uuid::Uuid;

#[derive(Queryable)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
}
