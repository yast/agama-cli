use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String
}

pub fn products() -> Vec<Product> {
    vec![
        Product {
            id: "Leap".to_string(),
            name: "openSUSE Leap 15.4".to_string(),
            description: "Leap uses source from SUSE Linux Enterprise (SLE), which...".to_string(),
        },
        Product {
            id: "ALP".to_string(),
            name: "SUSE ALP ContainerHost OS".to_string(),
            description: "'The Adaptable Linux Platform (ALP), the next generation of Linux...".to_string(),

        },
        Product {
            id: "Tumbleweed".to_string(),
            name: "openSUSE Tumbleweed".to_string(),
            description: "The Tumbleweed distribution is a pure rolling release version...".to_string(),
        }
    ]
}
