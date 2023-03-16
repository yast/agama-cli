use super::proxies::Questions1Proxy;
use zbus::Connection;
use zbus::fdo::ObjectManagerProxy;

/// D-Bus client for the manager service
pub struct QuestionsClient<'a> {
    questions_proxy: Questions1Proxy<'a>,
    object_manager_proxy: ObjectManagerProxy<'a>
}

impl<'a> QuestionsClient<'a> {
    pub async fn new(connection: Connection) -> zbus::Result<QuestionsClient<'a>> {
        Ok(Self {
            questions_proxy: Questions1Proxy::new(&connection).await?,
            object_manager_proxy: ObjectManagerProxy::builder(&connection)
                .destination("org.opensuse.DInstaller.Questions")?
                .path("/org/opensuse/DInstaller/Questions1")?.build().await?,
        })
    }

    pub async fn is_empty(&self) -> zbus::Result<bool> {
        let objects = self.object_manager_proxy.get_managed_objects().await?;

        Ok(objects.is_empty())
    }
}
