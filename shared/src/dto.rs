use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub email: String,
}
