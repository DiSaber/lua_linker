use std::fs;

pub fn link() -> bool {
    let mut main_file = match fs::read_to_string("./main.lua") {
        Ok(main_file) => main_file,
        Err(_) => {
            println!("Failed to read 'main.lua'");
            return false;
        }
    };

    let directory = match fs::read_dir("./") {
        Ok(directory) => directory,
        Err(_) => {
            println!("Failed to read the current directory");
            return false;
        }
    };

    for entry in directory {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if extension != "lua" {
                continue;
            }
        } else {
            continue;
        }

        let file_name = match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => file_name,
                None => continue,
            },
            None => continue,
        };

        let include_statement = "-- include ".to_owned() + file_name;

        if !main_file.contains(&include_statement) {
            continue;
        }

        let linking_file = match fs::read_to_string(&path) {
            Ok(linking_file) => linking_file,
            Err(_) => {
                println!("Failed to read '{}'", file_name);
                continue;
            }
        };

        println!("Linking -> {}", file_name);

        main_file = main_file.replace(&include_statement, &linking_file)
    }

    match fs::write("./final.lua", main_file) {
        Ok(_) => true,
        Err(_) => {
            println!("Failed to write to 'final.lua'");
            false
        }
    }
}
