use crate::{progress::Progress, proxies::Progress1Proxy};

use super::proxies::Manager1Proxy;
use zbus::blocking::Connection;

/// D-Bus client for the manager service
pub struct ManagerClient<'a> {
    manager_proxy: Manager1Proxy<'a>,
    progress_proxy: Progress1Proxy<'a>,
}

impl<'a> ManagerClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self {
            manager_proxy: Manager1Proxy::new(&connection)?,
            progress_proxy: Progress1Proxy::new(&connection)?,
        })
    }

    pub fn busy_services(&self) -> zbus::Result<Vec<String>> {
        self.manager_proxy.busy_services()
    }

    pub fn probe(&self) -> zbus::Result<()> {
        self.manager_proxy.probe()
    }

    pub fn install(&self) -> zbus::Result<()> {
        self.manager_proxy.commit()
    }

    pub fn can_install(&self) -> zbus::Result<bool> {
        self.manager_proxy.can_install()
    }

    pub fn progress(&self) -> zbus::Result<Progress> {
        Progress::from_proxy(&self.progress_proxy)
    }
}

