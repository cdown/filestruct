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

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
