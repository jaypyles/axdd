use daemonize::Daemonize;
use dirs::home_dir;
use env_logger;
use jrutils;
use log::LevelFilter;
use log::{info, warn};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

mod constants;

#[derive(Debug)]
struct Display {
    name: String,
    screenlayout: PathBuf,
}

/// Parse displays config file
fn parse_displays(config: &Path) -> Vec<Display> {
    let mut v: Vec<Display> = Vec::new();
    let parsed_toml = jrutils::toml::parse_toml(config);

    if let Some(toml) = parsed_toml.as_table() {
        for display in toml.values() {
            let name = match display.get("name") {
                Some(name) => name.as_str().unwrap(),
                None => panic!("Can't find name!"),
            };

            let screenlayout = match display.get("screenlayout") {
                Some(screenlayout) => {
                    let mut p = PathBuf::new();
                    p.push(screenlayout.as_str().unwrap());
                    p
                }
                None => panic!("Can't find screenlayout!"),
            };

            let d = Display {
                name: name.to_string(),
                screenlayout,
            };
            v.push(d);
        }
    }

    return v;
}

/// Check if a display is connected
fn check_connected(display: &Display) -> bool {
    let mut display_path = PathBuf::new();
    display_path.push(constants::DISPLAY_PATH);

    let dp = fs::read_dir(&display_path).expect("Couldn't read display path!");

    for entry_result in dp {
        let entry = entry_result.unwrap();
        let e = entry.file_name();
        let file_name = match e.to_str() {
            Some(file_name) => file_name,
            None => panic!("Couldn't get file name!"),
        };

        let display_name = "-".to_string() + &display.name; // oof

        if file_name.contains(&display_name) {
            return true;
        }
    }

    return false;
}

/// Get status of all available displays
fn get_available_displays() -> HashMap<String, String> {
    let mut display_map: HashMap<String, String> = HashMap::new();

    let mut display_path = PathBuf::new();
    display_path.push(constants::DISPLAY_PATH);

    let dp = fs::read_dir(&display_path).expect("Couldn't read display path!");

    for entry_result in dp {
        let entry = entry_result.unwrap().path();
        if entry.is_dir() {
            let read_dir = fs::read_dir(&entry).expect("Couldn't read directory!");

            for e in read_dir {
                let device_file = e.unwrap().path();
                let status_path = entry.join("status");

                if device_file == status_path {
                    let contents = fs::read_to_string(status_path).expect("Couldn't read file!");
                    let display_name = entry.file_name().unwrap().to_str().unwrap();
                    let d = String::from(display_name);
                    display_map.insert(d, contents);
                }
            }
        }
    }

    return display_map;
}

/// Ensure config folder and files exist
fn handle_config(config_dir: &Path, config: &Path) {
    jrutils::create_dir_if_not_exists(&config_dir); // create config folder if not exists
    jrutils::create_file_if_not_exists(&config, constants::EXAMPLE_TOML);
    // create config if doesn't exist
}

fn test_main() {
    let home_path = home_dir().expect("Could not get home dir!");
    let config = home_path.join(constants::CONFIG_PATH);
    let config_toml_path = config.join("config.toml");

    handle_config(&config, &config_toml_path);
    let mut current_status = get_available_displays();
    let displays = parse_displays(&config_toml_path);

    info!(target: "display_status", "Current Display Status: {:?}", current_status);

    loop {
        let checked = get_available_displays();

        let mut is_connected = false;

        info!(target: "display_status", "Current Display Status: {:?}", current_status);

        if !(checked == current_status) {
            for display in &displays {
                if check_connected(&display) {
                    info!("NEW DISPLAY CONNECTED: {:?}", display);
                    let home_path = home_dir().expect("Could not get home dir!");
                    let screen_path = home_path.join(&display.screenlayout);

                    Command::new("xrandr")
                        .arg("--auto")
                        .spawn()
                        .expect("Failed to execute xrandr");

                    Command::new("bash")
                        .arg(screen_path)
                        .spawn()
                        .expect("Failed to execute layout switch.");

                    current_status = get_available_displays();
                    is_connected = true;

                    info!(target: "display_status", "Current Display Status: {:?}", current_status);
                }
            }

            if !is_connected {
                Command::new("xrandr")
                    .arg("--auto")
                    .spawn()
                    .expect("Failed to execute xrandr");
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let home_path = home_dir().expect("Could not get home dir!");
    let config = home_path.join(constants::CONFIG_PATH);
    let config_toml_path = config.join("config.toml");

    handle_config(&config, &config_toml_path);
    let mut current_status = get_available_displays();
    let displays = parse_displays(&config_toml_path);

    info!(target: "display_status", "Current Display Status: {:?}", current_status);

    loop {
        let checked = get_available_displays();

        let mut is_connected = false;

        info!(target: "display_status", "Current Display Status: {:?}", current_status);

        if !(checked == current_status) {
            for display in &displays {
                if check_connected(&display) {
                    info!("NEW DISPLAY CONNECTED: {:?}", display);
                    let home_path = home_dir().expect("Could not get home dir!");
                    let screen_path = home_path.join(&display.screenlayout);

                    Command::new("xrandr")
                        .arg("--auto")
                        .spawn()
                        .expect("Failed to execute xrandr");

                    Command::new("bash")
                        .arg(screen_path)
                        .spawn()
                        .expect("Failed to execute layout switch.");

                    current_status = get_available_displays();
                    is_connected = true;

                    info!(target: "display_status", "Current Display Status: {:?}", current_status);
                }
            }

            if !is_connected {
                Command::new("xrandr")
                    .arg("--auto")
                    .spawn()
                    .expect("Failed to execute xrandr");
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}
