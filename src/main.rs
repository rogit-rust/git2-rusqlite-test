use git2;
use rusqlite;
use std::time::Instant;

fn main() {
    let start_one = Instant::now();
    for _ in 0..1000 {
        git2::Repository::open_bare(".git").unwrap();
    }
    let duration_one = start_one.elapsed();
    println!("Open git\t: {:?}", duration_one);

    let start_two = Instant::now();
    for _ in 0..1000 {
        rusqlite::Connection::open("1.db").unwrap();
    }
    let duration_two = start_two.elapsed();
    println!("Open sqlite\t: {:?}", duration_two);
}
