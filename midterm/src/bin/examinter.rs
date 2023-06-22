use std::io;
use std::io::prelude::*;

fn main() {
    /*
    north: 0
    south: 1
    east: 2
    west: 3
    */
    let places: Vec<String> = vec![
        "north".to_string(), 
        "south".to_string(),
        "east".to_string(),
        "west".to_string()
    ];
    let paths: Vec<[Option<usize>; 4]> = vec![
        [None, Some(1), Some(2), Some(3)], // North
        [Some(0), None, Some(2), Some(3)], // South
        [Some(0), Some(1), None, Some(3)], // East
        [Some(0), Some(1), Some(2), None] // West
    ];
    let mut inventory: Vec<String> = vec!["bread".to_string()];
    let mut available_stuff: Vec<Vec<String>> = vec![
        vec!["cheese".to_string(), "coin".to_string()],
        vec!["sword".to_string(), "apple".to_string(), "key".to_string()],
        vec!["shield".to_string()],
        vec!["helmet".to_string(), "boots".to_string()]
    ];
    let mut current_place = 0;

    let stdin = io::stdin();

    println!("Current place: {}", places[current_place]);
    println!("Available areas:");
    for place in paths[current_place] {
        match place {
            Some(p) => { println!("    {}", places[p]); }
            None => {}
        }
    }
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                if l == "quit" || l == "exit" {
                    break;
                } else if l.contains("go") {
                    let v: Vec<_> = l.split(' ').collect();
                    if v.len() > 2 {
                        match v[1] {
                            "north" => { current_place = 0; }
                            "south" => { current_place = 1; }
                            "east" => { current_place = 2; }
                            "west" => { current_place = 3; }
                            _=> { println!("Please go to an available area"); }
                        }
                    }
                    println!("Current place: {}", places[current_place]);
                    println!("Available areas:");
                    for place in paths[current_place] {
                        match place {
                            Some(p) => { println!("    {}", places[p]); }
                            None => {}
                        }
                    }
                    println!("Available items:");
                    for item in available_stuff[current_place].iter() {
                        println!("    {}", item);
                    }
                } else if l.contains("inventory") {
                    println!("Inventory:");
                    for item in inventory.iter() {
                        println!("    {}", item);
                    }
                } else if l.contains("take") {
                    let v: Vec<_> = l.split(' ').collect();
                    if v.len() >= 2 {
                        let mut i: Option<usize> = None;
                        for (index, s) in available_stuff[current_place].iter().enumerate() {
                            if s == v[1] { i = Some(index); }
                        }
                        if i.is_some() {
                            let item = available_stuff[current_place].swap_remove(i.unwrap());
                            inventory.push(item);
                        }
                    }
                } else if l.contains("drop") {
                    let v: Vec<_> = l.split(' ').collect();
                    if v.len() >= 2 {
                        let mut i: Option<usize> = None;
                        for (index, s) in inventory.iter().enumerate() {
                            if s == v[1] { i = Some(index); }
                        }
                        if i.is_some() {
                            let item = inventory.swap_remove(i.unwrap());
                            available_stuff[current_place].push(item);
                        }
                    }
                }
            }
            Err(e) => { dbg!(e); }
        }
    }
}