use crate::install_settings::SoftwareSettings;
use crate::software::SoftwareClient;
use std::error::Error;
use zbus::Connection;

/// Loads and stores the software settings from/to the D-Bus service.
pub struct SoftwareStore<'a> {
    software_client: SoftwareClient<'a>,
}

impl<'a> SoftwareStore<'a> {
    pub async fn new(connection: Connection) -> Result<SoftwareStore<'a>, zbus::Error> {
        Ok(Self {
            software_client: SoftwareClient::new(connection).await?,
        })
    }

    pub async fn load(&self) -> Result<SoftwareSettings, Box<dyn Error>> {
        let product = self.software_client.product().await?;

        Ok(SoftwareSettings {
            product: Some(product),
        })
    }

    pub async fn store(&self, settings: &SoftwareSettings) -> Result<(), Box<dyn Error>> {
        if let Some(product) = &settings.product {
            self.software_client.select_product(product).await?;
        }
        Ok(())
    }
}
