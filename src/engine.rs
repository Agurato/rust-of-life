use crate::ui;

use std::cmp::Ordering;
use rand::Rng;
use std::{thread, time};

pub fn randomize_map(map: &mut [bool]) {
    for cell in map.iter_mut() {
        let random = rand::thread_rng().gen_range(0, 4);
        match random.cmp(&1) {
            Ordering::Less => *cell = true,
            _ => *cell = false,
        }
    }
}

pub fn start_game(map: &mut [bool], width: usize) {
    loop {
        ui::display_map(map, width);

        // The map is a toroidal array (left is stitched to the right, and top is stitched to the bottom)
        let mut i: usize = 0;
        
        let mut new_map = vec![false; map.len()];

        for cell in map.iter() {
            let neighbor_sum: usize = get_neightbor_sum(map, width, i);
            
            if *cell {
                if neighbor_sum < 2 {
                    new_map[i] = false;
                }
                else if neighbor_sum > 3 {
                    new_map[i] = false;
                }
                else {
                    new_map[i] = true;
                }
            }
            else if neighbor_sum == 3{
                new_map[i] = true;
            }

            i += 1;
        }

        map.copy_from_slice(new_map.as_slice());

        if check_game_over(map) {
            break;
        }
        else {
            thread::sleep(time::Duration::from_millis(100));
        }
    }
    println!("GAME OVER!");
}

fn get_neightbor_sum(map: & [bool], width: usize, i: usize) -> usize {
    let height: usize = map.len()/width;

    let x: usize = i%width;
    let y: usize = i/width;

    let left_x: usize;
    let right_x: usize;
    let top_y: usize;
    let bottom_y: usize;

    let mut neighbor_sum: usize = 0;

    if x == 0 {
        left_x = width-1;
        right_x = x+1;
    }
    else if x == width-1 {
        left_x = x-1;
        right_x = 0;
    }
    else {
        left_x = x-1;
        right_x = x+1;
    }

    if y == 0 {
        top_y = height-1;
        bottom_y = y+1;
    }
    else if y == height-1 {
        top_y = y-1;
        bottom_y = 0;
    }
    else {
        top_y = y-1;
        bottom_y = y+1;
    }

    neighbor_sum += map[top_y*width + left_x] as usize
                    + map[top_y*width + x] as usize
                    + map[top_y*width + right_x] as usize
                    + map[y*width + left_x] as usize
                    + map[y*width + right_x] as usize
                    + map[bottom_y*width + left_x] as usize
                    + map[bottom_y*width + x] as usize
                    + map[bottom_y*width + right_x] as usize;

    return neighbor_sum;     
}

fn check_game_over(map: &mut [bool]) -> bool {
    let mut all_dead = true;
    for cell in map.iter() {
        if *cell {
            all_dead = false;
            break;
        }
    }
    return all_dead;
}