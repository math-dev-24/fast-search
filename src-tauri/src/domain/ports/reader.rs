use crate::domain::entities::file::File;
use crate::shared::errors::AppResult;

pub trait Reader {
    fn read(&self, file: &File) -> AppResult<String>;
}