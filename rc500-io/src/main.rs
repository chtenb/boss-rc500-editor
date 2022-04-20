use clap::Parser;
use std::fs::copy;
use std::path::Path;
use std::path::PathBuf;
use windows::core::Result as WindowsResult;
use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};
use windows::Devices::Portable::StorageDevice;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    working_dir: String,

    /// If the working directory already contains a config file, overwrite it
    #[clap(short, long)]
    overwrite: bool,
}

/// Alias for the numeric type that holds system exit codes.
pub type ExitCode = i32;

pub const OK: ExitCode = 0;
pub const ERROR: ExitCode = 1;

pub const DEVICE_NAME: &str = "BOSS_RC-500";

fn main() {
    let args = Args::parse();
    match list_devices() {
        Err(e) => {
            println!("Could not retrieve any device info: {:?}", e);
            std::process::exit(ERROR);
        },
        Ok(devs) => match ask_pull(&devs) {
            None => println!("No device chosen. Exiting."),
            Some(dev) => pull(&dev.path, &args.working_dir),
        },
    }
    std::process::exit(OK)
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
    let fetch_infos =
        DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::PortableStorageDevice)?;
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

fn pull(device_root: &str, destination: &str) -> () {
    let from = PathBuf::new()
        .join(device_root)
        .join(Path::new(r"ROLAND\DATA\MEMORY1.RC0"));
    let to = PathBuf::new()
        .join(destination)
        .join(Path::new(r"config.xml"));
    println!("Copying {:?} to {:?}", from, to);
    match copy(from, to) {
        Err(e) => println!("Error occurred while trying to copy data: {:?}", e),
        Ok(_v) => println!("Successfully pulled data"),
    }
}
