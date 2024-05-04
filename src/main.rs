use std::env;
use std::fs;
use std::process::Command;

fn print_phase(phase_name: &str, color_hex: &str) {
    let (r, g, b) = hex_to_rgb(color_hex);
    let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
    println!("{}{}━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m", color_code, phase_name);
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn install_package(package_name: &str) {
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to determine user's home directory");
            return;
        }
    };

    let scripts_dir = format!("{}/.config/pkg/scripts", home_dir);
    let script_path = format!("{}/{}.json", scripts_dir, package_name);

    let script_content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: Failed to read package script: {}", err);
            return;
        }
    };

    let package_info: serde_json::Value = match serde_json::from_str(&script_content) {
        Ok(info) => info,
        Err(err) => {
            eprintln!("Error: Failed to parse package script: {}", err);
            return;
        }
    };

    if let Some(install_steps) = package_info["install"].as_object() {
        if let Some(cloning_steps) = install_steps.get("cloning") {
            print_phase(" ", "F1502F");
            execute_commands(cloning_steps);
        }

        if let Some(building_steps) = install_steps.get("building") {
            print_phase(" building", "C3E88D");
            execute_commands(building_steps);
        }
    } else {
        eprintln!("Error: Invalid installation steps");
        return;
    }

    println!("Installed {} successfully!", package_name);
}

fn execute_commands(steps: &serde_json::Value) {
    if let serde_json::Value::Array(commands) = steps {
        for command in commands {
            if let Some(cmd) = command.as_str() {
                let _ = Command::new("sh").arg("-c").arg(cmd).status();
            }
        }
    }
}

fn uninstall_package(package_name: &str) {
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to determine user's home directory");
            return;
        }
    };

    let scripts_dir = format!("{}/.config/pkg/scripts", home_dir);
    let script_path = format!("{}/{}.json", scripts_dir, package_name);

    let script_content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: Failed to read package script: {}", err);
            return;
        }
    };

    let package_info: serde_json::Value = match serde_json::from_str(&script_content) {
        Ok(info) => info,
        Err(err) => {
            eprintln!("Error: Failed to parse package script: {}", err);
            return;
        }
    };

    if let serde_json::Value::Array(steps) = &package_info["uninstall"] {
        for step in steps {
            if let Some(command) = step.as_str() {
                let _ = Command::new("sh").arg("-c").arg(command).status();
            }
        }
    } else {
        eprintln!("Error: Invalid uninstallation steps");
        return;
    }

    println!("Uninstalled {} successfully!", package_name);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: pkg <install|uninstall> <package1> [<package2> ...]");
        std::process::exit(1);
    }

    let command = &args[1];
    let package_names = &args[2..];

    match command.as_str() {
        "install" => {
            for package_name in package_names {
                install_package(package_name);
            }
        },
        "uninstall" => {
            for package_name in package_names {
                uninstall_package(package_name);
            }
        },
        _ => {
            eprintln!("Error: Invalid command");
            std::process::exit(1);
        }
    }
}


