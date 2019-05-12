use std::io;
use std::cmp::Ordering;

pub fn get_user_size(size_name: String, min: usize) -> usize {
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

pub fn display_map(map: &[bool], width: usize) {
    println!();
    for _i in 0..width {
        print!("--");
    }
    println!();
    println!();
    let mut i: usize = 0;
    for cell in map.iter() {
        if *cell {
            print!("██");
        }
        else {
            print!("  ");
        }
        i += 1;
        if i == width {
            println!();
            i = 0;
        }
    }
    println!();
}