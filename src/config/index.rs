pub struct Config<'a> {
    search: &'a str,
    file_path: &'a str
}

impl Config<'_> {
    pub fn of<'a>(search: &'a str, file_path: &'a str) -> Config<'a> {
        Config {
            search,
            file_path
        }
    }
}