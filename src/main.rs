// fn main() {
//  panic!("Crash and terminate the porgram")   
// }

// fn main() {
//     let v: Vec<i32>= vec![1,2,4];
//     v[88];
//     println!("{}",v[99]);
// }

use std::fs::File;


fn main() {
    let opne_file = File::open("hello.txt");
    let _open_file_resutl = match opne_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
