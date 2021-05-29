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
        let power = get_value("Enter the points spent on the fighter's power:");
        let speed = get_value("Enter the points spent on the fighter's speed:");
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

impl<'a> FightObserver<'a> for Observer {
    fn new_round(&mut self, r: u8) {
        self.output(&format!("Round {}!", r));
    }

    fn new_turn(&mut self, r: u8) {
        self.output(&format!("Turn {}!", r));
    }

    fn speed_roll(
        &mut self,
        f: &'a Fighter,
        r1: StatValue,
        r2: StatValue,
        penalty: StatValue,
        result: StatValue,
    ) {
        self.output(&format!(
            "{} rolls {} & {}. Combined with current penalty of {} their speed roll is {}.",
            f.name(),
            r1,
            r2,
            penalty,
            result
        ));
    }

    fn declare_attacker(&mut self, f: &'a Fighter) {
        self.output(&format!("{} is attacking!", f.name()));
    }

    fn clinch(&mut self) {
        self.output("Clinch!");
    }

    fn attack_roll(
        &mut self,
        r1: StatValue,
        r2: StatValue,
        damage: StatValue,
        defender: &'a Fighter,
        new_health: StatValue,
    ) {
        self.output(&format!(
            "They roll {} & {} for {} damage. {} is now at {} hp.",
            r1,
            r2,
            damage,
            defender.name(),
            new_health
        ));
    }

    fn downed(&mut self, f: &'a Fighter) {
        self.output(&format!("{} is down!", f.name()));
    }

    fn getup_roll(
        &mut self,
        r1: StatValue,
        r2: StatValue,
        heal_amount: StatValue,
        new_health: StatValue,
    ) {
        self.output(&format!(
            "They roll {} & {}. They heal {} to {} hp.",
            r1, r2, heal_amount, new_health
        ));
    }

    fn interval(
        &mut self,
        f: &'a Fighter,
        current_health: StatValue,
        current_speed_penalty: StatValue,
    ) {
        self.output(&format!(
            "After resting {} is at {} hp with a {} speed penalty.",
            f.name(),
            current_health,
            current_speed_penalty
        ));
    }

    fn winner(&mut self, f: &'a Fighter) {
        self.output(&format!("{} wins!", f.name()));
    }
}
