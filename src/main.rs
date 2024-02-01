use std::fs;

fn main() {

    let image_ext: Vec<&str> = vec!["png", "jpg"];

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path_name = path.as_ref().unwrap().file_name();
        let path_name = path_name.to_str().unwrap();

        println!("path_name: {}", path_name);

        if path.unwrap().file_type().unwrap().is_file() {
            let ext: Vec<&str> = path_name.split(".").collect();

            if ext.len() > 1 {
                println!("ext: {}", ext[1]);
                if image_ext.contains(&ext[1]) {
                    println!("it is an image");
                } else {
                    println!("it is not an image");
                }
            }
        }
    }
}

// fn is_in(list: &Vec<&str>, target: &str) -> bool {
//     for ext in list {
//         if ext == &target {
//             return true;
//         }
//     }
//     false
// }
