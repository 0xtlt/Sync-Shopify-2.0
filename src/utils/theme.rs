use std::fs;

#[derive(Debug, Clone)]
pub struct ThemeFile {
    pub file_name: String,
    pub checksum: String,
    pub content: Vec<u8>,
    pub file_size: u64,
}

pub fn get_theme_content(theme_dir: &str) -> Vec<ThemeFile> {
    let mut files: Vec<ThemeFile> = vec![];

    // Read all files in the theme directory

    // /assets directory
    // Skip folder and .map files
    let assets_files = fs::read_dir(format!("{}/assets/", theme_dir)).unwrap();
    for file in assets_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with(".") || file_name.ends_with(".map") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("assets/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /config directory
    // Only sync settings_schema.json
    let config_files = fs::read_dir(format!("{}/config/", theme_dir)).unwrap();
    for file in config_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if file_name != "settings_schema.json" {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("config/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /layout directory
    // Only sync .liquid files
    let layout_files = fs::read_dir(format!("{}/layout/", theme_dir)).unwrap();
    for file in layout_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if !file_name.ends_with(".liquid") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("layout/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /locales directory
    // Only sync .json files
    // Don't sync it for the moment
    // TODO: add --sync-locales flag
    // let locales_files = fs::read_dir(format!("{}/locales/", theme_dir)).unwrap();
    // for file in locales_files {
    //   let entry = file.unwrap();
    //   let path = entry.path();

    //   if path.is_dir() {
    //     continue;
    //   }

    //   let file_name = entry.file_name().into_string().unwrap();
    //   if !file_name.ends_with(".json") {
    //     continue;
    //   }

    //   let file_content = fs::read_to_string(&path).unwrap();
    //   let file_size = fs::metadata(&path).unwrap().len();
    //   let file_checksum = format!("{:x}", md5::compute(&file_content));

    //   files.push(ThemeFile {
    //     file_name: format!("locales/{}", file_name),
    //     checksum: file_checksum,
    //     content: file_content,
    //     file_size,
    //   });
    // }

    // /sections directory
    // Only sync .liquid files
    let sections_files = fs::read_dir(format!("{}/sections/", theme_dir)).unwrap();
    for file in sections_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if !file_name.ends_with(".liquid") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("sections/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /snippets directory
    // Only sync .liquid files
    let snippets_files = fs::read_dir(format!("{}/snippets/", theme_dir)).unwrap();
    for file in snippets_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if !file_name.ends_with(".liquid") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("snippets/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /templates directory
    // Only sync .liquid files
    let templates_files = fs::read_dir(format!("{}/templates/", theme_dir)).unwrap();
    for file in templates_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if !file_name.ends_with(".liquid") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("templates/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    // /templates/customers/ directory
    // Only sync .liquid files
    let templates_customers_files =
        fs::read_dir(format!("{}/templates/customers/", theme_dir)).unwrap();
    for file in templates_customers_files {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if !file_name.ends_with(".liquid") {
            continue;
        }

        let file_content = fs::read(&path).unwrap();
        let file_size = fs::metadata(&path).unwrap().len();
        let file_checksum = format!("{:x}", md5::compute(&file_content));

        files.push(ThemeFile {
            file_name: format!("templates/customers/{}", file_name),
            checksum: file_checksum,
            content: file_content,
            file_size,
        });
    }

    files
}
