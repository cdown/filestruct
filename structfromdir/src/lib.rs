pub use structfromdir_derive::FromDir;

use thiserror::Error;
use std::path::PathBuf;

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
