use crate::domain::service::Service;
use crate::domain::service_repository::ServiceRepository;
use crate::infrastructure::systemd_service_adapter::ConnectionType;
use std::error::Error;
use std::collections::HashSet;
use std::env;

pub struct ServicesManager {
    repository: Box<dyn ServiceRepository>,
}

impl ServicesManager {
    pub fn new(repository: Box<dyn ServiceRepository>) -> Self {
        Self { repository }
    }

    pub fn start_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.start_service(service.name())?;
        Ok(service)
    }

    pub fn stop_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.stop_service(service.name())?;
        Ok(service)
    }

    pub fn restart_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.restart_service(service.name())?;
        Ok(service)
    }

    pub fn enable_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.enable_service(service.name())?;
        self.repository.reload_daemon()?;
        Ok(service)
    }

    pub fn disable_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.disable_service(service.name())?;
        self.repository.reload_daemon()?;
        Ok(service)
    }
    
    pub fn mask_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.mask_service(service.name())?;
        Ok(service)
    }

    pub fn unmask_service(&self, service: &Service) -> Result<Service, Box<dyn Error>> {
        let service = self.repository.unmask_service(service.name())?;
        Ok(service)
    }

    pub fn list_services(&self, filter: bool) -> Result<Vec<Service>, Box<dyn Error>> {
        let mut all = Vec::new();

        let mut services_runtime = self.repository.list_services(filter)?;
        let mut services_files = self.repository.list_service_files(filter)?;

        all.append(&mut services_runtime);
        all.append(&mut services_files);

        if filter {
            all.retain(|s| s.name().ends_with(".service"));
        }

        let mut seen = HashSet::new();
        all.retain(|s| seen.insert(s.name().to_string()));

        // Get the user's home directory to construct the user systemd config path
        let user_config_path = env::var("HOME")
            .map(|home| format!("{}/.config/systemd/user", home))
            .unwrap_or_else(|_| String::from("/.config/systemd/user"));

        // Sort with custom logic: prioritize services in user config directory, then by name
        all.sort_by(|a, b| {
            let a_is_user_config = a.state().path().starts_with(&user_config_path);
            let b_is_user_config = b.state().path().starts_with(&user_config_path);

            match (a_is_user_config, b_is_user_config) {
                (true, false) => std::cmp::Ordering::Less,    // a comes first
                (false, true) => std::cmp::Ordering::Greater, // b comes first
                _ => a.name().to_lowercase().cmp(&b.name().to_lowercase()), // both same priority, sort by name
            }
        });

        Ok(all)
    }

    pub fn get_log(&self, service: &Service) -> Result<String, Box<dyn Error>> {
        self.repository.get_service_log(service.name())
    }

    pub fn change_repository_connection(&mut self, connection_type: ConnectionType) -> Result<(), Box<dyn Error>> {
        self.repository.change_connection(connection_type)?;
        Ok(())
    }

    pub fn systemctl_cat(&self, service: &Service) -> Result<String, Box<dyn Error>> {
        self.repository.systemctl_cat(service.name())
    }
}

