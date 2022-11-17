pub mod login;
pub mod protocol;
pub mod socket;
pub mod state;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
