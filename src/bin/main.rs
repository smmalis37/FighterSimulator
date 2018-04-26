extern crate fighter_simulator;

use fighter_simulator::*;

fn main() {
    let f1 = Fighter::new("Alice".to_owned(), 8, 2, 4, 140).unwrap();
    let f2 = Fighter::new("Bob".to_owned(), 7, 3, 4, 125).unwrap();

    Fight::new(&f1, &f2).run_with_reporting(report_handler);
}

fn report_handler(report: &Report) {
    if let Some(new_round) = report.new_round {
        println!("Start of round {}.", new_round);
        println!();
    }

    for (attack, remaining_health) in report.attacks.iter().zip(report.remaining_healths.iter()) {
        if let Some(ref atk) = attack {
            println!(
                "{} attacked {} for {} damage.",
                atk.attacker.name, atk.defender.name, atk.damage
            );
            println!("First rolls were {:?}", atk.first_rolls);
            println!(
                "{}/{} survived {} endurance.",
                atk.second_rolls.len(),
                atk.first_rolls.len(),
                atk.defender.stats[&Stat::Endurance]
            );
            println!("Second rolls were {:?}", atk.second_rolls);
            println!(
                "{} now has {} health left.",
                atk.defender.name,
                remaining_health.unwrap()
            );
            println!();
        }
    }

    if let Some(ref winner) = report.winner {
        println!("{} wins!", winner.name);
    }
}
