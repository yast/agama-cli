use super::proxies::Software1Proxy;
use serde::Serialize;
use zbus::blocking::Connection;

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub struct SoftwareClient<'a> {
    pub connection: Connection,
    software_proxy: Software1Proxy<'a>,
}

impl<'a> SoftwareClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self {
            software_proxy: Software1Proxy::new(&connection)?,
            connection,
        })
    }

    pub fn products(&self) -> zbus::Result<Vec<Product>> {
        let products: Vec<Product> = self.software_proxy.available_base_products()?
            .into_iter().map(|(id, name, data)| {
                let description = match data.get("description") {
                    Some(value) => value.try_into().unwrap(),
                    None => ""
                };
                Product {
                    id,
                    name,
                    description: description.to_string()
                }
            }).collect();
        Ok(products)
    }

    pub fn product(&self) -> zbus::Result<String> {
        self.software_proxy.selected_base_product()
    }

    pub fn select_product(&self, product_id: &str) -> zbus::Result<()> {
        self.software_proxy.select_product(product_id)
    }
}
