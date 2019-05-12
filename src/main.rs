use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Conway's Game of Life!");

    let width: usize = get_user_size(String::from("width"), 10);
    let height: usize = get_user_size(String::from("height"), 10);
    
    let map = vec![vec![false; height]; width];
    println!("{}", map[0][0]);
}

fn get_user_size(size_name: String, min: usize) -> usize {
    let mut size: usize;

    loop {
        println!("Enter the {} of the map:", size_name);
        let mut width_input = String::new();

        io::stdin().read_line(&mut width_input)
            .expect("Failed to read line");

        size = match width_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match size.cmp(&min) {
            Ordering::Less => println!("The {} must be at least {}", size_name, min),
            _ => {
                break;
            }
        }
    }

    size
}