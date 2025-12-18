use std::path::PathBuf;
use rfd::FileDialog;
use crate::core::services::file_dialog_service::FileDialogService;

pub struct RfdFileDialogService {

}

impl FileDialogService for RfdFileDialogService {
    fn OpenImageDialog(&self) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("HDR image", &["hdr"])
            .add_filter("image", &["jpg", "png"])
            .set_directory("/")
            .pick_file()
    }
}

