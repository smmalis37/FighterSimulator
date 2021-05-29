use crate::fighter::*;
use crate::stats::*;

pub trait FightObserver<'a> {
    fn new_round(&mut self, r: u8);
    fn new_turn(&mut self, r: u8);
    fn speed_roll(
        &mut self,
        f: &'a Fighter,
        r1: StatValue,
        r2: StatValue,
        penalty: StatValue,
        result: StatValue,
    );
    fn declare_attacker(&mut self, f: &'a Fighter);
    fn clinch(&mut self);
    fn attack_roll(
        &mut self,
        r1: StatValue,
        r2: StatValue,
        damage: StatValue,
        defender: &'a Fighter,
        new_health: StatValue,
    );
    fn downed(&mut self, f: &'a Fighter);
    fn getup_roll(
        &mut self,
        r1: StatValue,
        r2: StatValue,
        heal_amount: StatValue,
        new_health: StatValue,
    );
    fn interval(
        &mut self,
        f: &'a Fighter,
        current_health: StatValue,
        current_speed_penalty: StatValue,
    );
}
