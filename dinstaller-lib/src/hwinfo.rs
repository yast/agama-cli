use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use zbus::zvariant::OwnedValue;

/// Represents the hardware information from the target system
///
/// TODO: by now, it just holds a little bit of information
#[derive(Debug, Serialize)]
pub struct HWInfo {
    pub disks: Vec<DiskInfo>,
    pub os_release: OSReleaseInfo,
}

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub size: u32,
}

#[derive(Debug, Serialize)]
pub struct OSReleaseInfo {
    pub id: String,
    pub name: String,
}

impl HWInfo {
    pub fn from_dbus(
        mut data: HashMap<String, zbus::zvariant::OwnedValue>,
    ) -> Result<HWInfo, Box<dyn Error>> {
        let disks: Vec<OwnedValue> = data.remove("disks").unwrap().try_into()?;
        let disks: Vec<DiskInfo> = disks
            .into_iter()
            .filter_map(|d| d.try_into().ok())
            .collect();
        let os_release: OSReleaseInfo = data.remove("os_release").unwrap().try_into()?;
        Ok(HWInfo { disks, os_release })
    }
}

impl TryFrom<OwnedValue> for DiskInfo {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let mut data: HashMap<String, OwnedValue> = value.try_into()?;
        let names: Vec<String> = data.remove("device_file").unwrap().try_into()?;

        let size: String = data.remove("size").unwrap().try_into()?;
        let size = match size.split_once(' ') {
            Some((size_str, _)) => size_str.parse().unwrap(),
            None => 0,
        };

        Ok(DiskInfo {
            name: names.get(0).unwrap().to_string(),
            size,
        })
    }
}

impl TryFrom<OwnedValue> for OSReleaseInfo {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let mut data: HashMap<String, OwnedValue> = value.try_into()?;
        let id: String = data.remove("id").unwrap().try_into()?;
        let name: String = data.remove("name").unwrap().try_into()?;

        Ok(OSReleaseInfo { id, name })
    }
}
