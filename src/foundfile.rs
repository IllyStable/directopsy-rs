use std::path::PathBuf;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FileType {
    Directory,
    File
}

impl From<&FileType> for &str {
    fn from(file_type: &FileType) -> &'static str {
        match *file_type {
            FileType::Directory => "Dir",
            FileType::File => "File"
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct FoundFile {
    pub path: PathBuf,
    pub file_type: FileType,
    pub name: String,
}

impl From<PathBuf> for FoundFile {
    fn from(path: PathBuf) -> FoundFile {
        let file_type = if path.is_file() { FileType::File } else { FileType::Directory };
        let file_name = path.file_name().unwrap_or_default().to_os_string().into_string().unwrap_or_default();

        FoundFile {
            path,
            file_type,
            name: file_name
        }
    }
}  
