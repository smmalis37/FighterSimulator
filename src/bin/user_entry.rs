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
        let attack = get_value("Enter the fighter's attack:");
        let speed = get_value("Enter the fighter's speed:");
        let endurance = get_value("Enter the fighter's endurance:");
        let max_health = get_value("Enter the fighter's max health:");

        match Fighter::new(name, attack, speed, endurance, max_health) {
            Ok(fighter) => break fighter,
            Err(e) => match e {
                FighterStatError::IncorrectPointTotal(total) => println!(
                    "That build uses {} points. You must use exactly {} points.",
                    total, TOTAL_POINTS
                ),
                FighterStatError::ZeroStat(stat) => {
                    println!("{:?} values of zero are not allowed.", stat)
                }
                FighterStatError::StatAboveMax(stat) => println!(
                    "{:?} values above {} are not allowed.",
                    stat,
                    stat.costs().len() - 1
                ),
                FighterStatError::HealthBelowBase => println!(
                    "{} is below the base health value of {}.",
                    max_health, BASE_HEALTH
                ),
                FighterStatError::HealthNotCleanlyDivisible => println!(
                    "{} is not cleanly divisible by the health per points value of {}.",
                    max_health, HEALTH_PER_POINT
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
    fn output<'a>(&mut self, text: &'a str) {
        writeln!(self.log_file, "{}", text).expect("Failed to write to log file.");
        println!("{}", text);
    }
}

impl<'a> FightObserver<'a> for Observer {
    fn new_round(&mut self, new_round: Round) {
        self.output(&format!("Start of round {}.", new_round));
    }
    fn attack_starting(&mut self, attacker: &'a Fighter, defender: &'a Fighter) {
        self.output(&format!("{} attacks {}.", attacker.name(), defender.name()));
    }
    fn first_roll(&mut self, roll: StatValue, success: bool) {
        self.output(&format!(
            "Fisrt roll of {} is {}good enough.",
            roll,
            if success { "" } else { "not " }
        ));
    }
    fn second_roll(&mut self, roll: StatValue) {
        self.output(&format!("Second roll is {}.", roll));
    }
    fn finalize_attack(&mut self, damage: StatValue, remaining_health: StatValue) {
        self.output(&format!(
            "Dealt {} damage. {} health remaining.",
            damage, remaining_health
        ));
    }
    fn winner(&mut self, winner: &'a Fighter) {
        self.output(&format!("{} wins!", winner.name()));
    }
}
