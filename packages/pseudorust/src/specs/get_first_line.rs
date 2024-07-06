use stubs::*;

pub fn get_first_line(file: File) -> FirstLine {
    todo!()
}

mod tests {
    use super::*;

    fn must_return_empty_line_if_file_is_empty() {
        let empty_file = create_temp_file();
        assert_eq!(get_first_line(empty_file), String::new())
    }
}

mod stubs {
    pub struct File;

    pub type FirstLine = String;

    pub fn create_temp_file() -> File {
        todo!()
    }
}
