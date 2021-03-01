use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new(id: Uuid) -> Self {
        Client { id }
    }
}
