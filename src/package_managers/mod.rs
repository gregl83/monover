mod npm;

use std::collections::HashMap;

pub(crate) trait PackageConfiguration {
    fn get_file_name(&self) -> String;
}

type Managers = HashMap<String, Box<dyn PackageConfiguration>>;

pub(crate) struct Collection {
    managers: Managers
}

impl Collection {
    fn construct_managers() -> Managers {
        let mut managers = HashMap::new();

        fn add_manager(managers: &mut Managers, manager: Box<dyn PackageConfiguration>) {
            managers.insert(manager.get_file_name(), manager);
        }

        add_manager(&mut managers, Box::new(nodejs::PackageJson::new()));

        managers
    }

    pub fn new() -> Self {
        Self {
            managers: Self::construct_managers()
        }
    }
}