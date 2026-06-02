pub mod config;
pub mod contact;
pub mod envy;
pub mod message;
pub mod token;
pub mod user;

pub mod init {
    pub fn is_uuid_nil(uuid: &Option<uuid::Uuid>) -> bool {
        match uuid {
            Some(id) => id.is_nil(),
            None => true,
        }
    }
}
