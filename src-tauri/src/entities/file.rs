use std::{path::PathBuf, time::SystemTime, fmt::{Display, Formatter}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub file_type: Option<String>,
    pub size: Option<u64>,
    pub last_modified: SystemTime,
    pub created_at: SystemTime,
    pub accessed_at: SystemTime,
    pub is_indexed: bool,
    pub content_indexed: bool,
    pub is_indexable: bool,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub is_system: bool,
    pub is_executable: bool,
    pub is_symlink: bool,
    pub permissions: Option<u32>,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
    pub line_count: Option<u32>,
    pub word_count: Option<u32>,
    pub checksum: Option<String>,
    pub is_encrypted: bool,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {{ path: {}, name: {}, is_dir: {}, file_type: {:?}, size: {:?}, last_modified: {:?}, created_at: {:?}, accessed_at: {:?}, is_indexed: {}, content_indexed: {}, is_indexable: {}, is_hidden: {}, is_readonly: {}, is_system: {}, is_executable: {}, is_symlink: {}, permissions: {:?}, owner: {:?}, group: {:?}, mime_type: {:?}, encoding: {:?}, line_count: {:?}, word_count: {:?}, checksum: {:?}, is_encrypted: {} }}",
            self.path.display(), self.name, self.is_dir, self.file_type, self.size, self.last_modified, self.created_at, self.accessed_at, self.is_indexed, self.content_indexed, self.is_indexable, self.is_hidden, self.is_readonly, self.is_system, self.is_executable, self.is_symlink, self.permissions, self.owner, self.group, self.mime_type, self.encoding, self.line_count, self.word_count, self.checksum, self.is_encrypted)
    }
}