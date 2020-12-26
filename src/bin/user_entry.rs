extern crate fighter_simulator;

use fighter_simulator::*;

use std::fs::*;
use std::io::*;
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
        let health = get_value("Enter the points spent on the fighter's health:");
        let skill = get_value("Enter the points spent on the fighter's skill:");
        let speed = get_value("Enter the points spent on the fighter's speed:");
        let strength = get_value("Enter the points spent on the fighter's strength:");
        let resist = get_value("Enter the points spent on the fighter's resist:");

        match Fighter::new(name, health, skill, speed, strength, resist) {
            Ok(fighter) => break fighter,
            Err(e) => match e {
                FighterStatError::IncorrectPointTotal(total) => println!(
                    "That build uses {} points. You must use exactly {} points.",
                    total, TOTAL_POINTS
                ),
                FighterStatError::StatAboveMax(stat) => println!(
                    "{:?} values above {} are not allowed.",
                    stat, MAX_STAT_POINTS
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

impl<'a> FightObserver<'a> for Observer {
    fn attack_starting(&mut self, attacker: &'a Fighter, defender: &'a Fighter) {
        self.output(&format!("{} attacks {}.", attacker.name(), defender.name()));
    }

    fn rolls(&mut self, rolls: &[StatValue]) {
        self.output(&format!("They roll:\n{:?}.", rolls));
    }

    fn adjusts(&mut self, rolls: &[StatValue]) {
        self.output(&format!(
            "After strength and resist adjustments, the rolls are:\n{:?}.",
            rolls
        ));
    }

    fn finalize_attack(&mut self, damage: StatValue, remaining_health: SignedStatValue) {
        self.output(&format!(
            "Dealt {} damage. {} health remaining.",
            damage, remaining_health
        ));
    }

    fn winner(&mut self, winner: Option<&'a Fighter>) {
        if let Some(w) = winner {
            self.output(&format!("{} wins!", w.name()));
        } else {
            self.output("Nobody wins!");
        }
    }
}
