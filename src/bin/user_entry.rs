extern crate fighter_simulator;

use fastrand::Rng;
use fighter_simulator::*;
use std::fs::File;
use std::io::{stdin, Write};
use std::str::FromStr;

pub fn main() {
    const TEAM_SIZE: usize = 1;
    println!("Team size is {}.", TEAM_SIZE);

    let t1 = [(); TEAM_SIZE].map(|_| {
        let f = get_fighter();
        println!("{} successfully registered.", f.name());
        println!();
        f
    });

    println!("Team 1 successfully registered.");
    println!();

    let t2 = [(); TEAM_SIZE].map(|_| {
        let f = get_fighter();
        println!("{} successfully registered.", f.name());
        println!();
        f
    });

    println!("Team 2 successfully registered.");
    println!();

    let filename = format!(
        "{}Vs{}.txt",
        t1.each_ref().map(|f| f.name().to_string()).join(","),
        t2.each_ref().map(|f| f.name().to_string()).join(",")
    );
    let mut file = File::create(filename).expect("Unable to create log file.");

    Fight::new(t1.each_ref(), t2.each_ref(), Rng::new().get_seed()).run(|s_fn| {
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
        let conviction = get_value("Enter the points spent on the fighter's conviction:");

        let fighter = Fighter::new(
            name, health, attack, defense, speed, accuracy, dodge, conviction,
        );

        if fighter.validate(true) {
            break fighter;
        } else if loop {
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
    };

    serde_json::to_writer(File::create(format!("{}.txt", f.name())).unwrap(), &f).unwrap();

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
