mod emoji;

use emoji::{get_emoji_list, Emoji};
use rand::Rng;
use std::env;

fn help() {
    println!(
        "Usage: emors [ all | select | random ]

Display all emoji list
$ emors all

Display the specified emoji
$ emors select <emoji name>

Display an emoji randamly
$ emors random

Display two emoji randamly
$ emors random 2

Display many emoji randamly
$ emors random 1000"
    );
}

fn emoji_random_display(num: usize, emoji_list: Vec<Emoji>) {
    for _ in 0..num {
        let rand_num = get_rand_num(emoji_list.len());
        print!("{}", emoji_list[rand_num].unicode);
    }

    // Needed to end a sentence with a line break
    println!();
}

fn emoji_select_display(name: String, emoji_list: Vec<Emoji>) {
    let mut iter = emoji_list.iter();
    match iter.find(|emoji| emoji.name == name.to_uppercase()) {
        None => {
            println!("No emoji name {} was found.", name);
            println!("===========================");
            println!("You can check the list of pictograms with this command.");
            println!("===========================");
            println!("$ emors all")
        }
        Some(emoji) => {
            println!("{}", emoji.unicode);
        }
    }
}

fn get_rand_num(max: usize) -> usize {
    rand::thread_rng().gen_range(0..max)
}

fn main() {
    let emoji_list: Vec<Emoji> = get_emoji_list();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help()
    } else {
        match args[1].as_str() {
            "random" => match args.len() {
                3 => {
                    match args[2].parse() {
                        Ok(num) => emoji_random_display(num, emoji_list),
                        Err(err) => eprintln!("{}", err),
                    };
                }
                2 => {
                    emoji_random_display(1, emoji_list);
                }
                _ => help(),
            },
            "select" => match args.len() {
                3 => {
                    match args[2].parse() {
                        Ok(name) => emoji_select_display(name, emoji_list),
                        Err(err) => eprintln!("{}", err),
                    };
                }
                2 => {
                    println!("You must specify the name of emoji");
                    println!("==================================");
                    println!("emors select \"WINKING FACE\"");
                }
                _ => {
                    println!("You must specify the name of emoji");
                    println!("==================================");
                    help();
                }
            },
            "all" => {
                for emoji in emoji_list {
                    println!("{}: {}", emoji.unicode, emoji.name);
                }
            }
            "help" => help(),
            _ => {
                println!("The specified command was not found.");
                println!("====================================");
                help();
            }
        }
    }
}
