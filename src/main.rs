use std::{fs, io, path::PathBuf};

mod folder_manager;

fn main() -> io::Result<()> {
    println!("===== 备份《下载》文件夹中的所有内容 =====");

    let source = folder_manager::get_source();

    let mut source_paths: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if should_skip(&path) {
            continue;
        }

        source_paths.push(path);
    }

    let destination = folder_manager::get_destination();

    println!("备份到《{}》", &destination);

    fs::create_dir_all(&destination)?;

    for source_file in source_paths {
        let mut destination_file = PathBuf::new();
        
        destination_file.push(&destination);

        match source_file.file_name() {
            Some(file_name) => destination_file.push(file_name),
            None => continue
        }

        fs::copy(source_file, destination_file)?;
    }

    Ok(())
}

fn should_skip(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_name) = ext.to_str() {
            if ext_name == "ini" {
                return true;
            }
        }
    }

    false
}
