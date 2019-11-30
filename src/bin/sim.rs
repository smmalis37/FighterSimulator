use fighter_simulator::*;
use rayon::prelude::*;

use std::sync::atomic::*;

fn main() {
    const FIGHT_COUNT: usize = 100;

    let time = std::time::SystemTime::now();
    let fighters = gen_fighters();
    let results = {
        let mut v = Vec::with_capacity(fighters.len());
        for _ in 0..fighters.len() {
            v.push(AtomicUsize::new(0));
        }
        v
    };

    println!("Simulating {} fighters.", fighters.len());

    fighters.par_iter().enumerate().for_each(|(i1, f1)| {
        for (i2, f2) in (i1 + 1..fighters.len()).map(|i2| (i2, &fighters[i2])) {
            for _ in 0..FIGHT_COUNT {
                let fight = Fight::new(f1, f2);
                let mut report = WinnerLogger { winner: None };
                fight.run(&mut report);
                let winner = report.winner.unwrap();

                if winner as *const _ == f1 as *const _ {
                    results[i1].fetch_add(1, Ordering::Relaxed);
                } else {
                    results[i2].fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    });

    let mut final_results = fighters
        .iter()
        .zip(results.into_iter().map(|w| w.into_inner()))
        .collect::<Vec<_>>();
    final_results.sort_by_key(|&(_, w)| w);

    for (f, w) in final_results {
        let loss_count = ((fighters.len() - 1) * FIGHT_COUNT) - w;
        let win_rate = w as f64 / (w + loss_count) as f64 * 100f64;
        println!(
            "{}\t{} wins\t{} losses\t{:.2}%",
            f.name(),
            w,
            loss_count,
            win_rate
        );
    }
    println!("{:?}", time.elapsed().unwrap());
}

fn gen_fighters() -> Vec<Fighter> {
    let mut fighters = Vec::new();

    for attack in 0..Stat::Attack.costs().len() {
        for speed in 0..Stat::Speed.costs().len() {
            for endurance in 0..Stat::Endurance.costs().len() {
                let stat_costs = Stat::Attack.costs()[attack]
                    + Stat::Speed.costs()[speed]
                    + Stat::Endurance.costs()[endurance];
                if stat_costs <= TOTAL_POINTS {
                    let health = (TOTAL_POINTS - stat_costs) * HEALTH_PER_POINT + BASE_HEALTH;
                    let name = format!("a{}s{}e{}h{}", attack, speed, endurance, health);

                    let maybe_fighter = Fighter::new(
                        name,
                        attack as StatValue,
                        speed as StatValue,
                        endurance as StatValue,
                        health,
                    );
                    if let Ok(fighter) = maybe_fighter {
                        fighters.push(fighter);
                    }
                }
            }
        }
    }

    fighters
}

struct WinnerLogger<'a> {
    winner: Option<&'a Fighter>,
}

impl<'a> FightObserver<'a> for WinnerLogger<'a> {
    fn new_round(&mut self, _: u16) {}
    fn attack_starting(&mut self, _: &'a Fighter, _: &'a Fighter) {}
    fn first_roll(&mut self, _: StatValue, _: bool) {}
    fn second_roll(&mut self, _: StatValue) {}
    fn finalize_attack(&mut self, _: StatValue, _: StatValue) {}
    fn winner(&mut self, winner: &'a Fighter) {
        self.winner = Some(winner);
    }
}
