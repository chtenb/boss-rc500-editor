use std::fs;
use std::path::Path;
use std::path::PathBuf;
use windows::core::Result as WindowsResult;
use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};
use windows::Devices::Portable::StorageDevice;

const DEVICE_NAME: &str = "BOSS_RC-500";

pub fn pull(working_dir: &str) -> Result<(), String> {
    match list_devices() {
        Err(e) => Err(format!("Could not retrieve any device info: {:?}", e).to_string()),
        Ok(devs) => match ask_pull(&devs) {
            None => {
                println!("No device chosen. Exiting.");
                Ok(())
            }
            Some(dev) => do_pull(&dev.path, working_dir),
        },
    }
}

#[derive(Clone, Debug)]
struct Device {
    name: String,
    path: String,
}

fn ask_pull(devs: &Vec<Device>) -> Option<Device> {
    for dev in devs {
        println!("{}: {}", dev.name, dev.path)
    }
    println!("");
    for dev in devs {
        if dev.name == DEVICE_NAME {
            println!("Pulling data from {}", dev.path);
            return Some(dev.clone());
        }
    }
    None
}

fn list_devices() -> WindowsResult<Vec<Device>> {
    let fetch_infos = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::PortableStorageDevice)?;
    let infos = fetch_infos.get()?;
    let nr_devs = infos.Size()?;
    println!("Found {:?} devices", nr_devs);
    let mut result: Vec<Device> = Vec::new();
    for info in infos {
        match scan_device(info) {
            Err(e) => println!("Error occurred while retrieving device info: {:?}", e),
            Ok(dev) => result.push(dev),
        }
    }
    Ok(result)
}

fn scan_device(info: DeviceInformation) -> WindowsResult<Device> {
    let name = info.Name()?;
    let id = info.Id()?;
    let storage_device = StorageDevice::FromId(id)?;
    let path = storage_device.Path()?;
    Ok(Device {
        name: name.to_string_lossy(),
        path: path.to_string_lossy(),
    })
}

fn do_pull(device_root: &str, destination: &str) -> Result<(), String> {
    let from = PathBuf::new()
        .join(device_root)
        .join(Path::new(r"ROLAND\DATA\MEMORY1.RC0"));
    let to = PathBuf::new().join(destination).join(Path::new(r"config.xml"));
    println!("Copying {:?} to {:?}", from, to);
    match fs::copy(from, to) {
        Err(e) => Err(format!("Error occurred while trying to copy data: {:?}", e)),
        Ok(_v) => {
            println!("Successfully pulled data");
            Ok(())
        }
    }
}
