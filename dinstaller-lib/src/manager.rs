use crate::hwinfo::HWInfo;
use crate::{progress::Progress, proxies::Progress1Proxy};
use std::error::Error;

use super::proxies::Manager1Proxy;
use zbus::Connection;

/// D-Bus client for the manager service
pub struct ManagerClient<'a> {
    manager_proxy: Manager1Proxy<'a>,
    progress_proxy: Progress1Proxy<'a>,
}

impl<'a> ManagerClient<'a> {
    pub async fn new(connection: Connection) -> zbus::Result<ManagerClient<'a>> {
        Ok(Self {
            manager_proxy: Manager1Proxy::new(&connection).await?,
            progress_proxy: Progress1Proxy::new(&connection).await?,
        })
    }

    pub async fn busy_services(&self) -> zbus::Result<Vec<String>> {
        self.manager_proxy.busy_services().await
    }

    pub async fn probe(&self) -> zbus::Result<()> {
        self.manager_proxy.probe().await
    }

    pub async fn install(&self) -> zbus::Result<()> {
        self.manager_proxy.commit().await
    }

    pub async fn can_install(&self) -> zbus::Result<bool> {
        self.manager_proxy.can_install().await
    }

    pub async fn progress(&self) -> zbus::Result<Progress> {
        Progress::from_proxy(&self.progress_proxy).await
    }

    pub async fn hwinfo(&self) -> Result<HWInfo, Box<dyn Error>> {
        let result = self.manager_proxy.hwinfo().await?;
        HWInfo::from_dbus(result)
    }
}
