use crate::fighter::*;
use crate::stats::*;

pub trait FightObserver<'a> {
    fn new_round(&mut self, r: usize);
    fn new_turn(&mut self, r: usize);
    fn attack(
        &mut self,
        attacker: &'a Fighter,
        defender: &'a Fighter,
        attack: Stat,
        damage: StatValue,
        new_health: StatValue,
    );
    fn stunned(&mut self, f: &'a Fighter);
    fn recovery(&mut self, f: &'a Fighter, new_health: StatValue);
}
