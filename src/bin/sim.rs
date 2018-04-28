extern crate fighter_simulator;

use fighter_simulator::*;

fn main() {
    let fighters = gen_fighters();
    let mut results = vec![(0, 0); fighters.len()];

    println!("Simulating {} fighters.", fighters.len());

    for (i1, f1) in fighters.iter().enumerate() {
        for (mut i2, f2) in fighters.iter().skip(i1 + 1).enumerate() {
            let i2 = i2 + i1 + 1;
            for _ in 0..10 {
                let fight = Fight::new(f1, f2);
                let winner = fight.run();

                if winner as *const _ == f1 as *const _ {
                    results[i1].0 += 1;
                    results[i2].1 += 1;
                } else {
                    results[i2].0 += 1;
                    results[i1].1 += 1;
                }
            }
        }
    }

    let mut final_results = fighters.iter().zip(results.iter()).collect::<Vec<_>>();
    final_results.sort_by_key(|&(_, w)| *w);
    for (f, w) in final_results {
        println!("{:?} {:?}", f, w);
    }
}

fn gen_fighters() -> Vec<Fighter> {
    let mut fighters = Vec::new();

    for attack in 0..Stat::Attack.costs().len() {
        for speed in 0..Stat::Speed.costs().len() {
            for endurance in 0..Stat::Endurance.costs().len() {
                let stat_costs = Stat::Attack.costs()[attack] + Stat::Speed.costs()[speed]
                    + Stat::Endurance.costs()[endurance];
                if stat_costs <= TOTAL_POINTS {
                    let health = (TOTAL_POINTS - stat_costs) * HEALTH_PER_POINT + BASE_HEALTH;
                    let name = format!("a{}s{}e{}h{}", attack, speed, endurance, health);

                    let maybe_fighter = Fighter::new(name, attack, speed, endurance, health);
                    if let Ok(fighter) = maybe_fighter {
                        fighters.push(fighter);
                    }
                }
            }
        }
    }

    fighters
}
