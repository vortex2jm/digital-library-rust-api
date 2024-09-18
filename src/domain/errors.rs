#[derive(Debug)]
pub enum DomainError {
    BookNotFound,
    UserNotFound,
    BookNotAvailable,
    BookNotRentedByUser,
    LockError,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DomainError {}
