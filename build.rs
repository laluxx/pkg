use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the home directory from the environment
    let home_dir = env::var("HOME").expect("Failed to find HOME directory");

    // Construct the path to the ~/.config/pkg directory
    let config_pkg_dir = Path::new(&home_dir).join(".config/pkg");

    // Check if the ~/.config/pkg directory exists and create it if it does not
    if !config_pkg_dir.exists() {
        fs::create_dir_all(&config_pkg_dir).expect("Failed to create pkg directory in .config");
    }

    // Path to the 'pkg' directory in the project
    let project_pkg_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("pkg");

    // Recursively copy from project 'pkg' directory to '~/.config/pkg'
    copy_dir_all(&project_pkg_dir, &config_pkg_dir).expect("Failed to copy pkg directory to .config");
}

/// Recursively copies a directory from a source path to a destination path.
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir(&dst)?;
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}
