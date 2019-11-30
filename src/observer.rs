use crate::fight::*;
use crate::fighter::*;
use crate::stats::*;

pub trait FightObserver<'a> {
    fn new_round(&mut self, new_round: Round);
    fn attack_starting(&mut self, attacker: &'a Fighter, defender: &'a Fighter);
    fn first_roll(&mut self, roll: StatValue, success: bool);
    fn second_roll(&mut self, roll: StatValue);
    fn finalize_attack(&mut self, damage: StatValue, remaining_health: StatValue);
    fn winner(&mut self, winner: &'a Fighter);
}
