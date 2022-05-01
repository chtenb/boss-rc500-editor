use std::fs;
use std::path::Path;
use std::path::PathBuf;
use windows::core::Result as WindowsResult;
use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};
use windows::Devices::Portable::StorageDevice;

const DEVICE_NAME: &str = "BOSS_RC-500";

pub fn pull(working_dir: &str, overwrite: bool) -> Result<String, String> {
    match list_devices() {
        Err(e) => Err(format!("Could not retrieve any device info: {:?}", e).to_string()),
        Ok(devs) => {
            let dev = pick_device(&devs)?;
            do_pull(&dev.path, working_dir)
        }
    }
}

pub fn push(working_dir: &str) -> Result<String, String> {
    match list_devices() {
        Err(e) => Err(format!("Could not retrieve any device info: {:?}", e).to_string()),
        Ok(devs) => {
            let dev = pick_device(&devs)?;
            do_push(&dev.path, working_dir)
        }
    }
}

#[derive(Clone, Debug)]
struct Device {
    name: String,
    path: String,
}

fn pick_device(devs: &Vec<Device>) -> Result<Device, String> {
    let rc500s: Vec<&Device> = devs.into_iter().filter(|dev| dev.name == DEVICE_NAME).collect();
    match rc500s[..] {
        [] => Err("No RC500 found".to_string()),
        [dev] => Ok(dev.clone()),
        _ => Err(format!(
            "Found {} RC500 devices. Don't know which to choose.",
            rc500s.len()
        )),
    }
}

pub fn print_devices() -> Result<(), String> {
    let fetch_infos = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::PortableStorageDevice)
        .map_err(|e| format!("{:?}", e))?;
    let infos = fetch_infos.get().map_err(|e| format!("{:?}", e))?;
    let nr_devs = infos.Size().map_err(|e| format!("{:?}", e))?;
    println!("Found {:?} devices", nr_devs);
    let mut result: Vec<Device> = Vec::new();
    for info in infos {
        match scan_device(info) {
            Err(e) => println!("Error occurred while retrieving device info: {:?}", e),
            Ok(dev) => result.push(dev),
        }
    }
    for dev in &result {
        println!("{}: {}", dev.name, dev.path)
    }
    Ok(())
}

fn list_devices() -> WindowsResult<Vec<Device>> {
    let fetch_infos = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::PortableStorageDevice)?;
    let infos = fetch_infos.get()?;
    let mut result: Vec<Device> = Vec::new();
    for info in infos {
        let dev = scan_device(info)?;
        result.push(dev);
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

fn do_pull(device_root: &str, working_dir: &str) -> Result<String, String> {
    let from = device_paths(device_root);
    let to = config_file_paths(working_dir);
    let msg = format!("Copying {:?} to {:?}", from, to);
    match fs::copy(from.0, to.0).and_then(|_| fs::copy(from.1, to.1)) {
        Err(e) => Err(format!("{}. Error occurred while trying to copy data: {:?}", msg, e)),
        Ok(_v) => Ok(format!("{}. Successfully pulled data", msg)),
    }
}

fn do_push(device_root: &str, working_dir: &str) -> Result<String, String> {
    let from = config_file_paths(working_dir);
    let to = device_paths(device_root);
    let msg = format!("Copying {:?} to {:?}", from, to);
    match fs::copy(from.0, to.0).and_then(|_| fs::copy(from.1, to.1)) {
        Err(e) => Err(format!("{}. Error occurred while trying to copy data: {:?}", msg, e)),
        Ok(_v) => Ok(format!("{}. Successfully pushed data", msg)),
    }
}

fn device_paths(device_root: &str) -> (PathBuf, PathBuf) {
    (
        PathBuf::new()
            .join(device_root)
            .join(Path::new(r"ROLAND\DATA\MEMORY1.RC0")),
        PathBuf::new()
            .join(device_root)
            .join(Path::new(r"ROLAND\DATA\MEMORY2.RC0")),
    )
}
pub fn config_file_paths(working_dir: &str) -> (PathBuf, PathBuf) {
    (
        PathBuf::new().join(working_dir).join(Path::new(r"MEMORY1.RC0")),
        PathBuf::new().join(working_dir).join(Path::new(r"MEMORY2.RC0")),
    )
}
