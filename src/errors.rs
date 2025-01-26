#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parse error: %s")]
    ParseError(String),

    #[error("error")]
    Something,
}
