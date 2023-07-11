struct PackageJson {
    _file_name: &str,
}

impl PackageJson {
    fn new() -> Self {
        Self {
            _file_name: "package.json",
        }
    }

    fn get_file_name(&self) -> String {
        String::from(self.name)
    }
}