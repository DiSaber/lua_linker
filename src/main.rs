use std::io::{stdin, Read};

pub mod linker;

fn main() {
    println!(
        "File linking {}!",
        if linker::link() {
            "completed"
        } else {
            "failed"
        }
    );
    stdin().read(&mut [0]).unwrap();
}
