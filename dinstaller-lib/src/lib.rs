use zbus::blocking::{ConnectionBuilder, Proxy};

pub fn products() -> Result<Vec<(String,String)>, zbus::Error> {
    let connection = ConnectionBuilder::address("unix:path=/run/dbus/system_bus_socket")?.build()?;
    let proxy = Proxy::new(&connection,
         "org.opensuse.DInstaller.Software",
          "/org/opensuse/DInstaller/Software1",
        "org.opensuse.DInstaller.Software1")?;
    proxy.get_property("AvailableBaseProducts")
}

mod dinstaller;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = products().expect("dbus failed");
        assert_eq!(result, 
            vec![
                (String::from("ALP"), String::from("SUSE ALP ContainerHost OS")),
                (String::from("Tumbleweed"), String::from("openSUSE Tumbleweed")),
                (String::from("Leap Micro"), String::from("openSUSE Leap Micro 5.3")),
                (String::from("Leap"), String::from("openSUSE Leap 15.4"))]);
    }
}
