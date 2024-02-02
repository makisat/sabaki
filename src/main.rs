use std::{fs, io::ErrorKind};

fn main() {

    let image_ext: Vec<&str> = vec!["png", "jpg", "jpeg", "gif"];

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path_name = path.as_ref().unwrap().file_name();
        let path_name = path_name.to_str().unwrap();

        println!("path_name: {}", path_name);

        if path.unwrap().file_type().unwrap().is_file() {
            let ext: Vec<&str> = path_name.split(".").collect();

            if ext.len() > 1 {
                println!("ext: {}", ext[1]);
                let ext_name = ext[1];
                if image_ext.contains(&ext_name) {
                    let res = fs::rename(path_name, "./images/".to_owned() + path_name);
                    if let Err(err) = res {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                fs::create_dir("./images").unwrap();
                                fs::rename(path_name, "./images/".to_owned() + path_name).expect(format!("error occured during moving {}", path_name).as_str());
                            },
                            _ => panic!("error occured during moving the files")
                        }
                    }
                }
            }
        }
    }
}

