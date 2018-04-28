extern crate fighter_simulator;

use fighter_simulator::*;

use std::fs::*;
use std::io::*;
use std::str::FromStr;

pub fn main() {
    let f1 = get_fighter();
    println!("{} successfully registered.", f1.name);
    println!();
    let f2 = get_fighter();
    println!("{} successfully registered.", f2.name);
    println!();

    let filename = f1.name.clone() + "Vs" + &f2.name + ".txt";
    let file = File::create(filename).expect("Unable to create log file.");

    Fight::new(&f1, &f2).run_with_reporting(|r| {
        report_handler(r, stdout());
        report_handler(r, &file);
    });
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

fn report_handler<T: Write>(report: &FullReport, mut output: T) {
    if let Some(new_round) = report.new_round {
        writeln!(output, "Start of round {}.", new_round).unwrap();
        writeln!(output).unwrap();
    }

    for (attack, remaining_health) in report.attacks.iter().zip(report.remaining_healths.iter()) {
        if let Some(ref atk) = *attack {
            writeln!(
                output,
                "{} attacked {} for {} damage.",
                atk.attacker.name, atk.defender.name, atk.damage
            ).unwrap();
            writeln!(output, "First rolls were {:?}.", atk.first_rolls).unwrap();
            writeln!(
                output,
                "{}/{} survived {} endurance.",
                atk.second_rolls.len(),
                atk.first_rolls.len(),
                atk.defender.stats[Stat::Endurance]
            ).unwrap();
            writeln!(output, "Second rolls were {:?}.", atk.second_rolls).unwrap();
            writeln!(
                output,
                "{} now has {} health left.",
                atk.defender.name,
                remaining_health.unwrap()
            ).unwrap();
            writeln!(output).unwrap();
        }
    }

    if let Some(ref winner) = report.winner {
        writeln!(output, "{} wins!", winner.name).unwrap();
    }
}
