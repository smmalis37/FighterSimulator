use fight::*;
use fighter::*;
use stats::*;

pub trait Report<'a> {
    fn new_round(&mut self, new_round: Round);
    fn attack(&mut self, attacker: &'a Fighter, defender: &'a Fighter);
    fn first_roll(&mut self, roll: StatValue);
    fn second_roll(&mut self, roll: StatValue);
    fn finalize_attack(&mut self, damage: StatValue, remaining_health: StatValue);
    fn winner(&mut self, winner: &'a Fighter);
}
