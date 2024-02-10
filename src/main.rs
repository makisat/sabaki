use std::{fs, io::ErrorKind};
use serde_json::Value;

fn main() {
    let ext_dir;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        match args[2].as_str() {
            "set" => {
                if args.len() < 3 {
                    println!("needs to specify ext_dir");
                    return;
                } 
                ext_dir = args[2];
            },
            other => println!("{} is not a command", other)
        }
    }

    let ext_val = read_json(ext_dir.as_str());
    let ext_obj = ext_val.as_object().unwrap();
    let ext_dir = ext_obj["ext_dir"].as_str().unwrap();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path_name = path.as_ref().unwrap().file_name();
        let path_name = path_name.to_str().unwrap();

        println!("content found: {}", path_name);

        if path.unwrap().file_type().unwrap().is_file() {
            let ext: Vec<&str> = path_name.split(".").collect();

            if ext.len() > 1 {
                let ext_name = ext[ext.len() - 1];

                for (key, val) in ext_obj {
                    let key_str = key.as_str();
                    let val_arr: Vec<&str> = val.as_array().unwrap().iter().map(|e| e.as_str().unwrap()).collect();

                    if val_arr.contains(&ext_name) {
                        sort_ext(&path_name, key_str);
                    }
                }
            }
        }
    }
}

fn sort_ext(path_name: &&str, dst: &str) {
    let res = fs::rename(path_name, format!("./{}/{}", dst, path_name));
    if let Err(err) = res {
        match err.kind() {
            ErrorKind::NotFound => {
                fs::create_dir("./".to_owned() + dst).unwrap();
                println!("dir '{}' created", dst);
                fs::rename(path_name, format!("./{}/{}", dst, path_name)).expect(format!("error occured during moving {}", path_name).as_str());
            },
            _ => panic!("error occured during moving the files")
        }
    }
    println!("content moved");
}

fn read_json(data_location: &str) -> Value {
    let extension_json = fs::read_to_string(data_location).unwrap();
    serde_json::from_str(&extension_json).unwrap()
}

