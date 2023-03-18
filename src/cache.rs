use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use log::info;


pub fn find_ancestor(name: &str) -> Option<PathBuf> {
    let current_dir = Path::new(".");
    
    for ancestor in current_dir.ancestors() {
        let path = ancestor.join(name);
        if path.is_dir() {
            return Some(path.to_path_buf());
        }
    }

    None
}


pub fn init_rvc() -> io::Result<()> {
    let git_path = find_ancestor(".git");
    let rvc_path = if let Some(git_path) = git_path {
        git_path.parent().unwrap().join(".rvc")
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "RVC works with git! Cannot find an active git repository"
        ));
    };

    if rvc_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "RVC directory already exists"
        ));
    }

    fs::create_dir(&rvc_path)?;
    let conf_path = rvc_path.join("config");
    let mut config_file = fs::File::create(&conf_path)?;
    let default_cache = r#"[cache]
    type = "reflink,hardlink,symlink"
"#;
    config_file.write_all(default_cache.as_bytes())?;
    info!("Initialized RVC!");
    Ok(())
}
