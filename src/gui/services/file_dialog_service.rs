use crate::core::services::file_dialog_service::FileDialogService;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct FileDialogServiceImpl {

}

impl FileDialogService for FileDialogServiceImpl {
    fn open_image(&self) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("HDR image", &["hdr"])
            .add_filter("image", &["jpg", "png"])
            .set_directory("/")
            .pick_file()
    }
}

