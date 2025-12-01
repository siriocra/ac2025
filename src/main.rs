mod day1;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(filename:String) -> String {
    let path_str = "input/".to_owned() + &filename;
    let path = Path::new(&path_str);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{}", path_str),
    }
    s
}

fn main() {
    let d1_file = "d1_p2.txt";
    let input = read_file(d1_file.to_string());
    println!("{}", day1::part2(input));
}