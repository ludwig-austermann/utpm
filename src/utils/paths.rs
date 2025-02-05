use std::{
    env::current_dir,
    fs::{read, read_dir, symlink_metadata},
};

use super::state::{ErrorState, Result};

#[cfg(not(feature = "portable"))]
pub fn get_data_dir() -> String {
    match dirs::data_local_dir() {
        Some(dir) => match dir.to_str() {
            Some(string) => String::from(string),
            None => String::from("/.local/share"), //default on linux
        },
        None => String::from("/.local/share"),
    }
}

pub fn get_home_dir() -> Result<String> {
    match dirs::home_dir() {
        Some(val) => match val.to_str() {
            Some(v) => Ok(String::from(v)),
            None => Err(ErrorState::HomeDirectoryError(String::from(
                "there is no home directory",
            ))),
        },
        None => Err(ErrorState::HomeDirectoryError(String::from(
            "there is no home directory",
        ))),
    }
}

pub fn get_ssh_dir() -> Result<String> {
    Ok(get_home_dir()? + "/.ssh")
}

#[cfg(feature = "portable")]
pub fn get_data_dir() -> String {
    get_current_dir().unwrap_or("./".to_string()) + "/utpmp"
}

pub fn d_packages() -> String {
    get_data_dir() + "/typst/packages"
}

pub fn datalocalutpm() -> String {
    get_data_dir() + "/utpm"
}

pub fn d_utpm() -> String {
    d_packages() + "/utpm"
}

pub fn get_current_dir() -> Result<String> {
    match current_dir() {
        Ok(val) => match val.to_str() {
            Some(v) => Ok(String::from(v)),
            None => Err(ErrorState::CurrentDirectoryError(String::from(
                "there is no current directory",
            ))),
        },
        Err(val) => Err(ErrorState::CurrentDirectoryError(val.to_string())),
    }
}

pub fn current_package() -> Result<String> {
    Ok(get_current_dir()? + "/typst.toml")
}

pub fn check_path_dir(path: &String) -> bool {
    read_dir(path).is_ok()
}

pub fn check_path_file(path: &String) -> bool {
    read(path).is_ok()
}

pub fn check_existing_symlink(path: &String) -> bool {
    let x = match symlink_metadata(path) {
        Ok(val) => val,
        Err(_) => return false,
    };
    x.file_type().is_symlink()
}
