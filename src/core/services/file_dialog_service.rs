use std::path::PathBuf;

pub trait FileDialogService {
    fn open_image(&self) -> Option<PathBuf>;
}