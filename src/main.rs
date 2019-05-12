mod engine;
mod ui;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    println!("Welcome to Conway's Game of Life!");
    
    let width: usize = ui::get_user_size(String::from("width"), 10);
    let height: usize = ui::get_user_size(String::from("height"), 10);

    let mut map = vec![false; width*height];
    
    engine::randomize_map(map.as_mut_slice());
    engine::start_game(map.as_mut_slice(), width);
}