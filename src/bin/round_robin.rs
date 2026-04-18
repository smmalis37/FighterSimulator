extern crate fighter_simulator;

use enum_map::Enum;
use fighter_simulator::*;
use rand::{RngExt, rng};

use std::fs::File;
use std::io::{Write, stdin};
use std::str::FromStr;

pub fn main() {
    let f_count = get_value("How many fighters?");
    let mut fighters = Vec::with_capacity(f_count);
    for _ in 0..f_count {
        let f = get_fighter();
        println!("{} successfully registered.", f.name());
        println!();
        fighters.push(f);
    }
    println!("All fighters successfully registered.");
    println!();

    let fights_per_matchup: usize = get_value("How many fights per matchup?");

    for i in 0..fighters.len() {
        for j in (i + 1)..fighters.len() {
            let f1 = &fighters[i];
            let f2 = &fighters[j];

            println!(
                "Running {} fights between {} and {}...",
                fights_per_matchup,
                f1.name(),
                f2.name()
            );

            for n in 0..fights_per_matchup {
                let filename = format!("{}Vs{}_{}.txt", f1.name(), f2.name(), n + 1);
                let mut file = File::create(&filename).expect("Unable to create log file.");
                Fight::new(f1, f2, rng().random()).run(|s_fn| {
                    let s = s_fn();
                    writeln!(file, "{}", s).expect("Failed to write to log file.");
                });
                println!("Completed fight {} of {}.", n + 1, fights_per_matchup);
            }
            println!();
        }
    }
}

fn get_fighter() -> Fighter {
    let name = get_value("Enter the fighter's name:");

    if let Some(f) = File::open(format!("{}.txt", name))
        .ok()
        .and_then(|f| serde_json::from_reader(f).ok())
    {
        return f;
    }

    println!("Available attack dice:");
    for i in 0..AttackDie::LENGTH {
        println!("{}:  {:?}", i, AttackDie::from_usize(i));
    }
    let attack = get_value("Enter the the fighter's attack die: ");
    let attack = AttackDie::from_usize(attack);

    println!("Available defense dice:");
    for i in 0..DefenceDie::LENGTH {
        println!("{}:  {:?}", i, DefenceDie::from_usize(i));
    }
    let defense = get_value("Enter the the fighter's defense die: ");
    let defense = DefenceDie::from_usize(defense);

    let f = Fighter::new(name, attack, defense);

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
