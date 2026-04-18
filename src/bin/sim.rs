use enum_map::Enum;
use fighter_simulator::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;
use std::{sync::atomic::*, time::Instant};

fn main() {
    const FIGHT_COUNT: usize = 1000;

    let time = Instant::now();
    let fighters = gen_fighters();
    let results = {
        let mut v = Vec::with_capacity(fighters.len());
        for _ in 0..fighters.len() {
            v.push((AtomicUsize::new(0), AtomicUsize::new(0)));
        }
        v
    };

    println!("Simulating {} fighters.", fighters.len());

    fighters.par_iter().enumerate().for_each(|(i1, f1)| {
        let mut rng = SmallRng::from_rng(&mut rand::rng());
        for (i2, f2) in (i1 + 1..fighters.len()).map(|i2| (i2, &fighters[i2])) {
            for _ in 0..FIGHT_COUNT {
                let fight = Fight::new(f1, f2, rng.random());
                let winner = fight.run(|_| {});

                if std::ptr::eq(winner, f1) {
                    results[i1].0.fetch_add(1, Ordering::Relaxed);
                    results[i2].1.fetch_add(1, Ordering::Relaxed);
                } else {
                    results[i2].0.fetch_add(1, Ordering::Relaxed);
                    results[i1].1.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    });

    let fight_count = (fighters.len() - 1) * FIGHT_COUNT;
    let mut final_results = fighters
        .iter()
        .zip(
            results
                .into_iter()
                .map(|(w, l)| (w.into_inner(), l.into_inner())),
        )
        .collect::<Vec<_>>();
    final_results.sort_by_key(|&(_, (w, _))| w);

    let final_time = Instant::now() - time;

    println!("attack,defense,wins,losses,");
    for (f, (w, l)) in final_results {
        assert!(w + l == fight_count);
        //let win_rate = (w as f64) / (fight_count as f64) * 100.0;
        println!("{},{},{},", f.name(), w, l);
    }

    println!("{:?}", final_time);
}

fn gen_fighters() -> Vec<Fighter> {
    let mut fighters = Vec::new();

    for attack in 0..AttackDie::LENGTH {
        for defense in 0..DefenceDie::LENGTH {
            let name = format!("{},{}", attack, defense);

            let fighter = Fighter::new(
                name,
                AttackDie::from_usize(attack),
                DefenceDie::from_usize(defense),
            );

            fighters.push(fighter);
        }
    }

    fighters
}
