

#[derive(Debug, thiserror::Error)]
pub enum SharedError {
    #[error("error")]
    Consistency,
}