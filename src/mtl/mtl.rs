use crate::{Fs, ImportError, Resource};
use crate::Path;

pub struct Mtl {}

impl Resource for Mtl {
    /**
    `Mtl::import`
    ---
    Import the `.mtl` file located at `path`.
    Returns the `Mtl` struct generated from
    said file, wrapped in a `Result`.
    */
    fn import<P>(path: P) -> Result<Self, ImportError>
    where
        P: AsRef<Path>,
    {
        let mut file = Fs::open_file(path)?;
        let text = Fs::read_file(&mut file)?;

        println!("{}", text);

        Fs::parse_lines(text, |_, _| {
            
            Ok(())
        })?;

        // Ok(Self { })
        todo!()
    }
}
