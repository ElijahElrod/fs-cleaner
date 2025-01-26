extern crate fs_extra;

use byte_unit::Byte;
use fs_extra::dir::get_size;
use std::{
    env,
    fs::{self},
    io,
    path::Path,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let top_level_dir = args.last().unwrap();
    let mut entries = fs::read_dir(top_level_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    dbg!(entries);

    let total_bytes_saved = visit_dirs(Path::new(top_level_dir.as_str()))?;
    let byte = Byte::from_u64(total_bytes_saved);
    println!("Cleaner freed {byte:#}");

    Ok(())
}

fn visit_dirs(dir: &Path) -> io::Result<u64> {
    let mut bytes_saved = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.as_os_str().to_str().expect("Could not get path_str");

            if path.is_dir() {
                match path_str.contains("/node_modules") {
                    true => {
                        let dir_size = get_size(&path).unwrap_or(0);
                        let byte = Byte::from_u64(dir_size);
                        println!(
                            "Deleting node_modules @ loc: {:?} with size {byte:#}",
                            path
                        );
                        let msg = match fs::remove_dir_all(&path) {
                            Err(_) => {
                                "Could not delete"
                            }
                            Ok(_) => {
                               "Deleted"
                            }
                        };
                        println!("{} node_modules @ loc: {:?}", msg, path);

                        bytes_saved += dir_size;
                    }
                    false => {
                        bytes_saved += visit_dirs(&path)?;
                    }
                }
            }
        }
    }

    Ok(bytes_saved)
}
