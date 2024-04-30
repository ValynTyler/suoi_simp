use crate::{Fs, ImportError};
use crate::Path;

pub struct Mtl {}

impl Mtl {
    /**
    `Mtl::import`
    ---
    Import the `.mtl` file located at `path`.
    Returns the `Mtl` struct generated from
    said file, wrapped in a `Result`.
    */
    pub fn import<P>(path: P) -> Result<Self, ImportError>
    // TODO: Create `Resource` trait to contain this and `Mtl`
    where
        P: AsRef<Path>,
    {
        let mut file = Fs::open_file(path)?;
        let _text = Fs::read_file(&mut file)?;

        todo!()
    }
}
