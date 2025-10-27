#[derive(Clone, Default, Debug)]
pub struct ServiceState {
    load: String,
    active: String,
    sub: String,
    file: String,
    path: String,
}

impl ServiceState {
    pub fn new(load: String, active: String, sub: String, file: String, path: String) -> Self {
        ServiceState {
            load,
            active,
            sub,
            file,
            path,
        }
    }

    pub fn load(&self) -> &str {
        &self.load
    }

    pub fn active(&self) -> &str {
        &self.active
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
