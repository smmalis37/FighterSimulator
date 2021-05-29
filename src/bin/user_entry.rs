extern crate fighter_simulator;

use fighter_simulator::*;

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
    let file = File::create(filename).expect("Unable to create log file.");

    Fight::new(&f1, &f2).run(&mut Observer { log_file: file });
}

fn get_fighter() -> Fighter {
    loop {
        let name = get_value("Enter the fighter's name:");
        let speed = get_value("Enter the points spent on the fighter's speed:");
        let power = get_value("Enter the points spent on the fighter's power:");
        let toughness = get_value("Enter the points spent on the fighter's toughness:");

        match Fighter::new(name, speed, power, toughness) {
            Ok(fighter) => break fighter,
            Err(e) => match e {
                FighterStatError::IncorrectPointTotal(total) => println!(
                    "That build uses {} points. You must use exactly {} points.",
                    total, TOTAL_POINTS
                ),
                FighterStatError::StatAboveMax(stat) => println!(
                    "{:?} values above {} are not allowed.",
                    stat, MAX_STAT_VALUE
                ),
                FighterStatError::StatBelowMin(stat) => println!(
                    "{:?} values below {} are not allowed.",
                    stat, MIN_STAT_VALUE
                ),
            },
        }
    }
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

struct Observer {
    log_file: File,
}

impl Observer {
    fn output(&mut self, text: &str) {
        writeln!(self.log_file, "{}", text).expect("Failed to write to log file.");
        println!("{}", text);
    }
}

impl<'a> FightObserver<'a> for Observer {}
