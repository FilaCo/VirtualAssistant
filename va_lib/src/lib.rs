#[derive(Debug)]
pub enum VAError {}

pub type VAResult<T> = Result<T, VAError>;
