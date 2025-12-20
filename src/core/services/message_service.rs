pub trait MessageService {
    fn show_info(&self, title: String, message: String);

    fn show_warning(&self, title: String, message: String);

    fn show_error(&self, title: String, message: String);

    fn show_exception(&self, title: String, message: String);
}