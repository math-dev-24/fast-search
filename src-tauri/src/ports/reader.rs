use crate::entities::file::File;

pub trait Reader {
    fn read(&self, file: &File) -> Result<String, String>;
}