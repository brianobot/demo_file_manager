use std::fs::create_dir_all;
use std::{fs, io};
use std::path::Path;


fn main() {
    println!("Welcome to the 🗂️ File Manager");

    loop {
        println!("
            Enter 1 to create Directory
            Enter 2 to create File
            Enter 3 to Remove Directory
            Enter 4 to Remove File
            Enter 0 to Close the Program
        ");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command).expect("Can not read command");
        
        let command = command.trim().parse::<u8>().unwrap();
        
        let result = match command {
            1 => create_directory(),
            2 => create_file(),
            3 => remove_directory(),
            4 => remove_file(),
            0 => {
                println!("Closing Program");
                break
            }
            _ => {
                println!("Invalid Option");
                Ok(())
            },
        };

        match result {
            Ok(_) => continue,
            Err(msg) => println!("❌ Error Message: {}", msg),
        }
    }
}


fn get_path_str() -> String {
    println!("Enter Path String: ");
    let mut path = String::new();

    io::stdin().read_line(&mut path).expect("Count not read path value");

    let path = path.trim().to_string();
    return path;
}


fn create_directory() -> Result<(), String> {
    let path = get_path_str();
    let directory_exist = Path::new(&path).exists();
    if directory_exist {
        return Err("Directory Already Exist".to_string());
    }
    
    Ok(fs::create_dir(path).unwrap())
}


fn create_file() -> Result<(), String> {
    let path = get_path_str();
    let mut parent_path = get_parent_path(&path);
    println!("Parent path: {}", parent_path);
    let current_file = path.split("/").last().expect("Can not Convert Path to str");
    println!("Current File: {}", current_file);

    create_dir_all(parent_path.clone()).expect("Could not create all directories");

    match fs::create_dir(&format!("{}", parent_path.clone())) {
        Ok(_) => println!("Create Base Directory ✅"),
        Err(msg) => println!("Failed to create Base Directory, error: {msg:?}"),
    }

    if parent_path.is_empty() {
        parent_path = String::from(".");
    }

    match fs::write(format!("{parent_path}/{current_file}"), b"") {
        Ok(_) => println!("Create File ✅"),
        Err(msg) => println!("Failed to create file, error: {msg:?}"),
    }

    Ok(())
}

fn remove_directory() -> Result<(), String> {
    let path = get_path_str();
    let directory_exist = Path::new(&path).exists();

    if !directory_exist {
        return Err("Directory does not exist".to_string());
    }

    let result = match fs::remove_dir(path) {
        Ok(_) => Ok(()),
        Err(msg) => Err(msg.to_string())
    };

    result
}


fn remove_file() -> Result<(), String> {
    let path = get_path_str();

    if !Path::new(&path).exists() {
        return Err(String::from("Path does not exist"));
    };

    Ok(fs::remove_file(path).unwrap())
}


fn get_parent_path(path: &str) -> String {
    let components = path.split("/").collect::<Vec<&str>>();

    let parent_path = components.iter()
        .map(|x| *x)
        .take(components.len() - 1)
        .collect::<Vec<&str>>()
        .join("/");

    return parent_path;
} 