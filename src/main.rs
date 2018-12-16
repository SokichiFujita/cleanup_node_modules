use std::fs;

fn main() {
    let dir = "./";
    let mut paths = fs::read_dir(dir).unwrap();

    println!("Display files of {}", dir);

    loop {
        match paths.next() {
            Some(path) => {
                println!("{:?}", path.unwrap().path());
            },
            None => { break }
        }
    }
}
