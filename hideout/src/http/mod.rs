mod cookie;
pub mod form_data;
pub mod request;
mod response;
mod status;
mod uri;

pub use cookie::Cookie;
pub use form_data::FormData;
pub use request::{Method, Request};
pub use response::Response;
pub use status::StatusCode;
pub use uri::Uri;

pub enum Error {
    Unauthorized { uri: String },
    NotFound { uri: String },
}

impl Error {
    pub fn status_code(&self) -> i32 {
        match self {
            Error::Unauthorized { .. } => 401,
            Error::NotFound { .. } => 404,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
