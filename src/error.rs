// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

#[derive(thiserror::Error)]
pub enum Error {
    #[error("file doesn't have parent path: {0}")]
    InvalidPath(std::path::PathBuf),
}

// pretty-print the error
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
