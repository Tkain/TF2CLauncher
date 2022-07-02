#![windows_subsystem = "windows"]

use std::ffi::OsString;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use simple_error::{bail, SimpleError};
use windows::Win32::System::Registry::{HKEY_LOCAL_MACHINE, RegGetValueW, RRF_RT_REG_SZ};

fn main() {

    let steam_path = get_steam_path().expect("Unable to locate Steam installation");
    let sdk_path = get_sdk_path(steam_path.as_path()).expect("Unable to locate Source SDK installation");
    let tf2c_path = steam_path.join("steamapps").join("sourcemods").join("tf2classic");

    std::process::Command::new(sdk_path).arg("-game").arg(tf2c_path).arg("-steam").arg("-steam").args(std::env::args())
        .spawn().expect("Unable to launch Source SDK executable");
}

type ExtResult<T> = Result<T, Box<dyn std::error::Error>>;

fn get_steam_path() -> ExtResult<PathBuf> {

    const MAX_PATH_LENGTH: usize = 260;

    let mut data = vec![0u16; MAX_PATH_LENGTH];
    let mut size = (size_of::<u16>() * MAX_PATH_LENGTH) as u32;

    let query = unsafe { RegGetValueW(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Valve\\Steam",
        "InstallPath",
        RRF_RT_REG_SZ,
        std::ptr::null_mut(),
        data.as_mut_ptr() as *mut std::ffi::c_void,
        &mut size as *mut u32
    ) };

    query.ok()?;

    let length = (size as usize) / size_of::<u16>() - 1;
    let string: OsString = std::os::windows::ffi::OsStringExt::from_wide(&data[..length]);
    let path = PathBuf::from(string);

    path.metadata()?;

    Ok(path)
}

fn get_sdk_path(steam_path: &Path) -> ExtResult<PathBuf> {

    let libraries_path = steam_path.to_path_buf().join("config").join("libraryfolders.vdf");
    let libraries_string = std::fs::read_to_string(libraries_path)?;
    let libraries_data = keyvalues_parser::Vdf::parse(&*libraries_string)?;

    let parse_error = || SimpleError::new("Unable to parse Steam library folder list");

    let libraries = libraries_data.value.get_obj().ok_or_else(parse_error)?.values();
    for library in libraries {
        let node = library
            .get(0).ok_or_else(parse_error)?
            .get_obj().ok_or_else(parse_error)?;
        let apps = node
            .get("apps").ok_or_else(parse_error)?
            .get(0).ok_or_else(parse_error)?
            .get_obj().ok_or_else(parse_error)?;
        if apps.contains_key("243750") {
            let path_string = node
                .get("path").ok_or_else(parse_error)?
                .get(0).ok_or_else(parse_error)?
                .get_str().ok_or_else(parse_error)?;
            let path = PathBuf::from(path_string)
                .join("steamapps").join("common")
                .join("Source SDK Base 2013 Multiplayer").join("hl2.exe");
            path.metadata()?;
            return Ok(path);
        }
    }

    bail!("SDK not installed in any known libraries")
}
