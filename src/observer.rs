use crate::fighter::*;
use crate::stats::*;

pub trait FightObserver<'a> {
    fn attack_starting(&mut self, attacker: &'a Fighter, defender: &'a Fighter);
    fn rolls(&mut self, rolls: &[StatValue]);
    fn adjusts(&mut self, rolls: &[StatValue]);
    fn finalize_attack(&mut self, damage: StatValue, remaining_health: SignedStatValue);
    fn winner(&mut self, winner: Option<&'a Fighter>);
}
