use crate::Resource;

pub struct Png {}

impl Resource for Png {
    fn import(path: &crate::Path) -> Result<Self, crate::ImportError>
    where
        Self: Sized,
    {
        


        Ok(Self {})
    }
}
