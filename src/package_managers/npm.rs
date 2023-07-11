use regex::Regex;
use self::super::PackageConfiguration;

pub(crate) struct PackageJson {
    _file_pattern: Regex,
}

impl PackageJson {
    pub(crate) fn new() -> Self {
        Self {
            _file_pattern: Regex::new(r"package\.json").unwrap()
        }
    }
}

impl PackageConfiguration for PackageJson {
    fn is_file_match(&self, file_name: &str) -> bool {
        self._file_pattern.is_match(file_name)
    }
}