use std::path::Path;
use std::collections::HashMap;
use zbus::zvariant::Value;
use zbus::dbus_proxy;

pub fn connection() -> Result<zbus::blocking::Connection, zbus::Error>{
    let path = if Path::new("/run/d-installer/bus").exists() {
        "/run/d-installer/bus"
    } else {
        "/run/dbus/system_bus_socket"
    };
    let address = format!("unix:path={}", path);
    zbus::blocking::ConnectionBuilder::address(address.as_str())?.build()
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Software1",
    default_service = "org.opensuse.DInstaller.Software",
    default_path = "/org/opensuse/DInstaller/Software1",
    gen_async = false
)]
trait SoftwareInterface {
    #[dbus_proxy(property)]
    fn available_base_products(&self) -> zbus::fdo::Result<Vec<(String, String, HashMap<String, Value>)>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let connection = connection().unwrap();
        let proxy = SoftwareInterfaceProxy::builder(&connection).build().unwrap();
        let result = proxy.available_base_products().unwrap();
        let result: Vec<String> = result.iter().map(|x| x.0.clone()).collect();
        assert_eq!(result, 
            vec![
                String::from("ALP"),
                String::from("Tumbleweed"),
                String::from("Leap Micro"),
                String::from("Leap")
            ]
        );
    }
}