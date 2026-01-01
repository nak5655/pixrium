use std::path::PathBuf;

pub trait DialogService {
    fn open_image(&self) -> Option<PathBuf>;

    #[allow(unused)]
    fn show_info(&self, title: String, message: String);

    #[allow(unused)]
    fn show_warning(&self, title: String, message: String);

    #[allow(unused)]
    fn show_error(&self, title: String, message: String);

    #[allow(unused)]
    fn show_exception(&self, title: String, message: String);
}