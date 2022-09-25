extern crate fighter_simulator;

use fighter_simulator::*;
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::{stdin, Write};
use std::str::FromStr;

pub fn main() {
    let f1 = get_fighter();
    println!("{} successfully registered.", f1.name());
    println!();
    let f2 = get_fighter();
    println!("{} successfully registered.", f2.name());
    println!();

    let filename = format!("{}Vs{}.txt", f1.name(), f2.name());
    let mut file = File::create(filename).expect("Unable to create log file.");

    Fight::new(&f1, &f2, thread_rng().gen()).run(|s_fn| {
        let s = s_fn();
        println!("{}", s);
        writeln!(file, "{}", s).expect("Failed to write to log file.");
    });
}

fn get_fighter() -> Fighter {
    let f = loop {
        let name = get_value("Enter the fighter's name:");

        if let Some(f) = File::open(&format!("{}.txt", name))
            .ok()
            .and_then(|f| serde_json::from_reader(f).ok())
        {
            return f;
        }

        let health = get_value("Enter the points spent on the fighter's health:");
        let attack = get_value("Enter the points spent on the fighter's attack:");
        let defense = get_value("Enter the points spent on the fighter's defense:");
        let speed = get_value("Enter the points spent on the fighter's speed:");
        let accuracy = get_value("Enter the points spent on the fighter's accuracy:");
        let dodge = get_value("Enter the points spent on the fighter's dodge:");

        let fighter = Fighter::new(name, health, attack, defense, speed, accuracy, dodge);

        if fighter.validate(true) {
            break fighter;
        } else {
            if loop {
                let mut buf = String::new();
                println!("Ok? (y/n)");
                stdin().read_line(&mut buf).unwrap();
                let yn = buf.trim().to_ascii_lowercase();

                if yn == "y" {
                    break true;
                } else if yn == "n" {
                    break false;
                }
            } {
                break fighter;
            }
        }
    };

    serde_json::to_writer(File::create(&format!("{}.txt", f.name())).unwrap(), &f).unwrap();

    f
}

fn get_value<T: FromStr>(prompt: &str) -> T {
    loop {
        let mut buffer = String::new();
        println!("{}", prompt);
        let read_attempt = stdin()
            .read_line(&mut buffer)
            .map(|_| buffer.trim().parse());
        if let Ok(Ok(value)) = read_attempt {
            break value;
        } else {
            println!("Invalid input.");
        }
    }
}
