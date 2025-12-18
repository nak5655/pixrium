use std::path::PathBuf;

pub trait FileDialogService {
    fn OpenImageDialog(&self) -> Option<PathBuf>;
}