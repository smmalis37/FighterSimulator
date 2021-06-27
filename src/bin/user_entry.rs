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

    let round_count = get_value("How many rounds:");
    let turn_count = get_value("How many turns per round:");

    let filename = format!("{}Vs{}.txt", f1.name(), f2.name());
    let file = File::create(filename).expect("Unable to create log file.");

    let mut o = Observer { log_file: file };
    let winner = Fight::new(&f1, &f2, round_count, turn_count).run(&mut o);

    match winner {
        Some(f) => o.output(&format!("{} wins!", f.name())),
        None => o.output("It's a draw!"),
    }
}

fn get_fighter() -> Fighter {
    loop {
        let name = get_value("Enter the fighter's name:");
        let health = get_value("Enter their health:");
        let jab = get_value("Enter their jab:");
        let hook = get_value("Enter their hook:");
        let straight = get_value("Enter their straight:");
        let uppercut = get_value("Enter their uppercut:");
        let special = get_value("Enter their special:");
        let recovery = get_value("Enter their recovery:");

        match Fighter::new(
            name, health, jab, hook, straight, uppercut, special, recovery,
        ) {
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
    pub fn output(&mut self, text: &str) {
        writeln!(self.log_file, "{}", text).expect("Failed to write to log file.");
        println!("{}", text);
    }
}

impl<'a> FightObserver<'a> for Observer {
    fn new_round(&mut self, r: usize) {
        self.output(&format!("Round {}", r));
    }

    fn new_turn(&mut self, t: usize) {
        self.output(&format!("Turn {}", t));
    }

    fn attack(
        &mut self,
        attacker: &'a Fighter,
        defender: &'a Fighter,
        attack: Stat,
        damage: StatValue,
        new_health: StatValue,
    ) {
        self.output(&format!(
            "{} hits {} with a {:?} for {} damage. {} is now at {} health.",
            attacker.name(),
            defender.name(),
            attack,
            damage,
            defender.name(),
            new_health
        ));
    }

    fn stunned(&mut self, f: &'a Fighter) {
        self.output(&format!("{} is stunned and does nothing", f.name()));
    }

    fn recovery(&mut self, f: &'a Fighter, new_health: StatValue) {
        self.output(&format!("{} heals to {}", f.name(), new_health));
    }
}
