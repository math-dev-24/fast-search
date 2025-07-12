
pub trait Reader {
    fn read(&self, path: &str) -> Result<String, String>;
}