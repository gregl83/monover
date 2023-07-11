mod npm;

pub(crate) trait PackageConfiguration {
    fn is_file_match(&self, file_name: &str) -> bool;
}

type Managers = Vec<Box<dyn PackageConfiguration>>;

pub(crate) struct Collection {
    managers: Managers
}

impl Collection {
    pub fn new() -> Self {
        Self {
            managers: vec![
                Box::new(npm::PackageJson::new())
            ]
        }
    }

    pub fn has_file_match(&self, file_name: &str) -> bool {
        for package_configuration in self.managers.iter() {
            if package_configuration.is_file_match(&file_name) {
                return true;
            }
        }
        false
    }
}