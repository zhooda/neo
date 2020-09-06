use chrono::{Local, Timelike};

// fn get_i32_hour() -> i32 {
//     let hour = chrono::Local::now().ho
// }

fn get_time_string() -> String {
    let now = Local::now();
    let hour = now.hour() as i32;

    if 0 <= hour && hour <= 4 {
        String::from("Good night")
    } else if 5 <= hour && hour <= 11 {
        String::from("Good morning")
    } else if 12 <= hour && hour <= 16 {
        String::from("Good afternoon")
    } else if 17 <= hour && hour <= 20 {
        String::from("Good evening")
    } else if 21 <= hour && hour <= 23 {
        String::from("Good night")
    } else {
        String::from("Knock, knock")
    }
    //
    // match hour {
    //     0..=4 => "night".to_string(),
    //     5..=11 => "morning".to_string(),
    //     12..=16 => "afternoon".to_string(),
    //     17..=20 => "evening".to_string(),
    //     21..=23 => "night".to_string()
    // }

}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("USAGE:\n\t{} <name>", args[0]);
        std::process::exit(1);
    }

    // println!("Knock, knock, {}", args[1]);
    println!("{}, {}", get_time_string(), args[1]);
}
