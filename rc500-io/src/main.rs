use clap::Parser;
// use core::result;
// use core::result::Result;
use std::fs::copy;
use std::path::Path;
use std::path::PathBuf;
use windows::core::Result;
use windows::Devices::Enumeration::{DeviceClass, DeviceInformation, DeviceInformationCollection};
use windows::Devices::Portable::StorageDevice;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    // let foo = StorageFile::GetFileFromPathAsync("test");
    // println!("{:?}", foo);
    // let bar = StorageDevice::GetDeviceSelector();
    // println!("{:?}", bar)
    // StorageDevice::GetDeviceSelector()
    //     .map

    let fetch_infos =
        DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::PortableStorageDevice)?;
    let infos = fetch_infos.get()?;
    let nr_devs = infos.Size()?;
    println!("Found {:?} devices", nr_devs);
    for info in infos {
        print_info(info)
            .map_err(|e| {
                println!("Error occurred while retrieving device info: {:?}", e)
            });
    }
    Ok(())
}

fn copy_files(device_root: &Path, to: &Path) -> Result<()> {
    let from = PathBuf::new().join(device_root).join(Path::new(r"ROLAND\DATA\MEMORY1.RC0"));
    println!("Copying {:?} to {:?}", from, to);
    let result = copy(from, to);
    println!("{:?}", result)
}

fn print_info(info: DeviceInformation) -> Result<()> {
    let name = info.Name()?;
    let id = info.Id()?;
    println!("{:?} {:?}", id, name);
    let storage = StorageDevice::FromId(id)?;
    let path = storage.Path()?;
    println!("Path: {}", path);
    Ok(())
}
