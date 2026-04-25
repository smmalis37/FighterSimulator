extern crate fighter_simulator;

use enum_map::Enum;
use fighter_simulator::*;
use rand::{RngExt, rng};
use std::fs::File;
use std::io::{Write, stdin};
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

    Fight::new(&f1, &f2, rng().random()).run(|s_fn| {
        let s = s_fn();
        println!("{}", s);
        writeln!(file, "{}", s).expect("Failed to write to log file.");
    });
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

    let cutman: YesNo = get_value("Does the fighter have a cut man? (y/n)");
    let strength_training: YesNo = get_value("Does the fighter have strength training? (y/n)");
    let impenetrable_guard: YesNo = get_value("Does the fighter have an impenetrable guard? (y/n)");
    let champions_resilience: YesNo =
        get_value("Does the fighter have champion's resilience? (y/n)");

    let f = Fighter::new(
        name,
        attack,
        defense,
        cutman.0,
        strength_training.0,
        impenetrable_guard.0,
        champions_resilience.0,
    );

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

struct YesNo(bool);

impl FromStr for YesNo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "y" | "yes" => Ok(YesNo(true)),
            "n" | "no" => Ok(YesNo(false)),
            _ => Err(()),
        }
    }
}
