use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CustomError {
    pub(crate) message: String,
}