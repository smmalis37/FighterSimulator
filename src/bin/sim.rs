use fighter_simulator::*;
use rayon::prelude::*;

use std::{sync::atomic::*, time::Instant};

fn main() {
    const FIGHT_COUNT: usize = 1000;

    let time = Instant::now();
    let fighters = gen_fighters();
    let results = {
        let mut v = Vec::with_capacity(fighters.len());
        for _ in 0..fighters.len() {
            v.push((
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
            ));
        }
        v
    };

    println!("Simulating {} fighters.", fighters.len());

    fighters.par_iter().enumerate().for_each(|(i1, f1)| {
        for (i2, f2) in (i1 + 1..fighters.len()).map(|i2| (i2, &fighters[i2])) {
            for _ in 0..FIGHT_COUNT {
                let fight = Fight::new(f1, f2, 12, 5);
                let winner = fight.run(&mut NoneLogger);

                if let Some(winner) = winner {
                    if std::ptr::eq(winner, f1) {
                        results[i1].0.fetch_add(1, Ordering::Relaxed);
                        results[i2].2.fetch_add(1, Ordering::Relaxed);
                    } else {
                        results[i2].0.fetch_add(1, Ordering::Relaxed);
                        results[i1].2.fetch_add(1, Ordering::Relaxed);
                    }
                } else {
                    results[i1].1.fetch_add(1, Ordering::Relaxed);
                    results[i2].1.fetch_add(1, Ordering::Relaxed);
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
                .map(|(w, t, l)| (w.into_inner(), t.into_inner(), l.into_inner())),
        )
        .collect::<Vec<_>>();
    final_results.sort_by_key(|&(_, (w, _, _))| w);

    let final_time = Instant::now() - time;

    println!("power,speed,toughness\twins,ties,losses");
    for (f, (w, t, l)) in final_results {
        assert!(w + t + l == fight_count);
        //let win_rate = (w as f64) / (fight_count as f64) * 100.0;
        println!("{}\t{},{},{}", f.name(), w, t, l);
    }

    println!("{:?}", final_time);
}

fn gen_fighters() -> Vec<Fighter> {
    let mut fighters = Vec::new();

    for jab in MIN_STAT_VALUE..=MAX_STAT_VALUE {
        for hook in MIN_STAT_VALUE..=MAX_STAT_VALUE {
            for straight in MIN_STAT_VALUE..=MAX_STAT_VALUE {
                for uppercut in MIN_STAT_VALUE..=MAX_STAT_VALUE {
                    for special in MIN_STAT_VALUE..=MAX_STAT_VALUE {
                        for recovery in MIN_STAT_VALUE..=MAX_STAT_VALUE {
                            let name = format!(
                                "{},{},{},{},{},{}",
                                jab, hook, straight, uppercut, special, recovery
                            );

                            if let Ok(fighter) = Fighter::new(
                                name, 100, jab, hook, straight, uppercut, special, recovery,
                            ) {
                                fighters.push(fighter);
                            }
                        }
                    }
                }
            }
        }
    }

    fighters
}

struct NoneLogger;

impl<'a> FightObserver<'a> for NoneLogger {
    fn new_round(&mut self, _: usize) {}
    fn new_turn(&mut self, _: usize) {}
    fn attack(&mut self, _: &'a Fighter, _: &'a Fighter, _: Stat, _: StatValue, _: StatValue) {}
    fn stunned(&mut self, _: &'a Fighter) {}
    fn recovery(&mut self, _: &'a Fighter, _: StatValue) {}
    fn down(&mut self, _: &'a Fighter) {}
    fn getup(&mut self, _: &'a Fighter, _: StatValue) {}
}
