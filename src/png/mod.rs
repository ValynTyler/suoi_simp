use crate::{Fs, Resource};

pub struct Png {}

impl Resource for Png {
    fn import(path: &crate::Path) -> Result<Self, crate::ImportError>
    where
        Self: Sized,
    {
        let mut file = Fs::open_file(path)?;
        let bytes = Fs::read_bytes(&mut file)?;
        
        println!("{:x?}", bytes);

        Ok(Self {})
    }
}
