pub mod config;
pub mod contact;
pub mod envy;
pub mod message;
pub mod token;
pub mod user;

pub mod init {
    pub fn is_uuid_nil(uuid: &uuid::Uuid) -> bool {
        uuid.is_nil()
    }
}
