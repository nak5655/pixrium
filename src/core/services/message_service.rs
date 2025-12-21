pub trait MessageService {
    #[allow(unused)]
    fn show_info(&self, title: String, message: String);

    #[allow(unused)]
    fn show_warning(&self, title: String, message: String);

    #[allow(unused)]
    fn show_error(&self, title: String, message: String);

    #[allow(unused)]
    fn show_exception(&self, title: String, message: String);
}