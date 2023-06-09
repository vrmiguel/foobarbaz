use actix_web::ResponseError;
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP server error: {0}")]
    Actix(#[from] actix_web::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("DB error: {0}")]
    SeaOrm(#[from] sea_orm::DbErr),
    #[error("Resource not found")]
    NotFound,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
