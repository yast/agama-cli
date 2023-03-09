use super::proxies::Software1Proxy;
use serde::Serialize;
use zbus::Connection;

/// Represents a software product
#[derive(Debug, Serialize)]
pub struct Product {
    /// Product ID (eg., "ALP", "Tumbleweed", etc.)
    pub id: String,
    /// Product name (e.g., "openSUSE Tumbleweed")
    pub name: String,
    /// Product description
    pub description: String,
}

/// D-Bus client for the software service
pub struct SoftwareClient<'a> {
    software_proxy: Software1Proxy<'a>,
}

impl<'a> SoftwareClient<'a> {
    pub async fn new(connection: Connection) -> zbus::Result<SoftwareClient<'a>> {
        Ok(Self {
            software_proxy: Software1Proxy::new(&connection).await?,
        })
    }

    /// Returns the available products
    pub async fn products(&self) -> zbus::Result<Vec<Product>> {
        let products: Vec<Product> = self
            .software_proxy
            .available_base_products().await?
            .into_iter()
            .map(|(id, name, data)| {
                let description = match data.get("description") {
                    Some(value) => value.try_into().unwrap(),
                    None => "",
                };
                Product {
                    id,
                    name,
                    description: description.to_string(),
                }
            })
            .collect();
        Ok(products)
    }

    /// Returns the selected product to install
    pub async fn product(&self) -> zbus::Result<String> {
        self.software_proxy.selected_base_product().await
    }

    /// Selects the product to install
    pub async fn select_product(&self, product_id: &str) -> zbus::Result<()> {
        self.software_proxy.select_product(product_id).await
    }
}
