use std::io::{stdin, Read};

pub mod linker;

fn main() {
    println!(
        "File Linking {}",
        if linker::link() {
            "Completed"
        } else {
            "Failed"
        }
    );
    stdin().read(&mut [0]).unwrap();
}
