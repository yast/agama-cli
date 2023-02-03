use crate::{progress::Progress, proxies::Progress1Proxy};

use super::proxies::Manager1Proxy;
use zbus::blocking::Connection;

pub struct ManagerClient<'a> {
    manager_proxy: Manager1Proxy<'a>,
    progress_proxy: Progress1Proxy<'a>
}

impl<'a> ManagerClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self { 
            manager_proxy: Manager1Proxy::new(&connection)?,
            progress_proxy: Progress1Proxy::new(&connection)?
        })
    }

    pub fn busy_services(&self) -> zbus::Result<Vec<String>> {
        self.manager_proxy.busy_services()
    }

    pub fn probe(&self) -> zbus::Result<()> {
        self.manager_proxy.probe()
    }

    pub fn progress(&self) -> zbus::Result<Progress> {
        Progress::from_proxy(&self.progress_proxy)
    }
}