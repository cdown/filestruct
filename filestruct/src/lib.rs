pub use filestruct_derive::FromDir;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{file}: cannot parse {input} as a {ty}")]
    Parse {
        file: PathBuf,
        input: String,
        ty: String,
    },

    #[error("{file}: {err}")]
    Io { file: PathBuf, err: std::io::Error },

    #[error(transparent)]
    IoPlain(#[from] std::io::Error),
}
