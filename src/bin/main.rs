extern crate fighter_simulator;

use fighter_simulator::*;

fn main() {
    let f1 = Fighter::new("Alice".to_owned(), 8, 2, 4, 140).unwrap();
    let f2 = Fighter::new("Bob".to_owned(), 6, 2, 5, 50).unwrap();

    Fight::new(&f1, &f2).run_with_reporting(report_handler);
}

fn report_handler(report: &Report) {
    if let Some(new_round) = report.new_round {
        println!("Start of round {}.", new_round)
    }

    for attack in &report.attacks {
        if let Some(ref atk) = attack {
            println!(
                "{} attacked {} for {} damage.",
                atk.attacker.name, atk.defender.name, atk.damage
            );
        }
    }

    if let Some(ref winner) = report.winner {
        println!("{} wins!", winner.name);
    }
}
