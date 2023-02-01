use super::proxies::Manager1Proxy;
use zbus::blocking::Connection;

pub struct ManagerClient<'a> {
    manager_proxy: Manager1Proxy<'a>,
}

impl<'a> ManagerClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self { 
            manager_proxy: Manager1Proxy::new(&connection)?
        })
    }

    pub fn busy_services(&self) -> zbus::Result<Vec<String>> {
        self.manager_proxy.busy_services()
    }

    pub fn probe(&self) -> zbus::Result<()> {
        self.manager_proxy.probe()
    }
}