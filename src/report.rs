use fight::*;
use fighter::*;
use stats::*;

use arrayvec::*;

type AttackRolls = ArrayVec<[StatValue; 20]>;

#[derive(Debug)]
pub struct FullReport<'a> {
    pub new_round: Option<Round>,
    pub attacks: [Option<FullAttackReport<'a>>; 2],
    pub winner: Option<&'a Fighter>,
}

#[derive(Debug)]
pub struct FullAttackReport<'a> {
    pub attacker: &'a Fighter,
    pub defender: &'a Fighter,
    pub first_rolls: AttackRolls,
    pub second_rolls: AttackRolls,
    pub damage: StatValue,
    pub remaining_health: StatValue,
}

pub(crate) struct WinnerOnlyReport<'a> {
    pub winner: Option<&'a Fighter>,
}

pub(crate) trait Report<'a> {
    fn new() -> Self;
    fn get_winner(&self) -> Option<&'a Fighter>;

    fn set_winner(&mut self, winner: &'a Fighter);
    fn set_new_round(&mut self, new_round: Round);
    fn set_attack(&mut self, attack_index: usize, attacker: &'a Fighter, defender: &'a Fighter);
    fn set_first_roll(&mut self, attack_index: usize, roll_index: usize, roll: StatValue);
    fn set_second_roll(&mut self, attack_index: usize, roll_index: usize, roll: StatValue);
    fn finalize_attack(&mut self, attack_index: usize, remaining_health: StatValue);
}

impl<'a> Report<'a> for FullReport<'a> {
    fn new() -> Self {
        FullReport {
            new_round: None,
            attacks: [None, None],
            winner: None,
        }
    }

    fn get_winner(&self) -> Option<&'a Fighter> {
        self.winner
    }

    fn set_winner(&mut self, winner: &'a Fighter) {
        self.winner = Some(winner);
    }

    fn set_new_round(&mut self, new_round: Round) {
        self.new_round = Some(new_round);
    }

    fn set_attack(&mut self, attack_index: usize, attacker: &'a Fighter, defender: &'a Fighter) {
        self.attacks[attack_index] = Some(FullAttackReport {
            attacker,
            defender,
            first_rolls: ArrayVec::new(),
            second_rolls: ArrayVec::new(),
            damage: 0,
            remaining_health: 987654321,
        });
    }

    fn set_first_roll(&mut self, attack_index: usize, roll_index: usize, roll: StatValue) {
        self.attacks[attack_index]
            .as_mut()
            .unwrap()
            .first_rolls
            .insert(roll_index, roll);
    }

    fn set_second_roll(&mut self, attack_index: usize, roll_index: usize, roll: StatValue) {
        let attack = self.attacks[attack_index].as_mut().unwrap();
        attack.second_rolls.insert(roll_index, roll);
        attack.damage += roll;
    }

    fn finalize_attack(&mut self, attack_index: usize, remaining_health: StatValue) {
        let attack = self.attacks[attack_index].as_mut().unwrap();
        attack.remaining_health = remaining_health;
    }
}

impl<'a> Report<'a> for WinnerOnlyReport<'a> {
    fn new() -> Self {
        WinnerOnlyReport { winner: None }
    }

    fn get_winner(&self) -> Option<&'a Fighter> {
        self.winner
    }

    fn set_winner(&mut self, winner: &'a Fighter) {
        self.winner = Some(winner);
    }

    fn set_new_round(&mut self, _new_round: Round) {}

    fn set_attack(&mut self, _attack_index: usize, _attacker: &'a Fighter, _defender: &'a Fighter) {
    }

    fn set_first_roll(&mut self, _attack_index: usize, _roll_index: usize, _roll: StatValue) {}

    fn set_second_roll(&mut self, _attack_index: usize, _roll_index: usize, _roll: StatValue) {}

    fn finalize_attack(&mut self, _attack_index: usize, _remaining_health: StatValue) {}
}
