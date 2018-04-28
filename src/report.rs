use arrayvec::ArrayVec;

use fight::Round;
use fighter::*;
use stats::*;

#[derive(Debug)]
pub struct FullReport<'a> {
    pub new_round: Option<Round>,
    pub attacks: [Option<AttackReport<'a>>; 2],
    pub remaining_healths: [Option<StatValue>; 2],
    pub winner: Option<&'a Fighter>,
}

#[derive(Debug)]
pub struct AttackReport<'a> {
    pub attacker: &'a Fighter,
    pub defender: &'a Fighter,
    pub first_rolls: ArrayVec<[StatValue; 20]>,
    pub second_rolls: ArrayVec<[StatValue; 20]>,
    pub damage: StatValue,
}

pub(crate) struct WinnerOnlyReport<'a> {
    pub winner: Option<&'a Fighter>,
}

pub(crate) trait Report<'a> {
    fn new() -> Self;
    fn get_winner(&self) -> Option<&'a Fighter>;

    fn set_winner(&mut self, winner: &'a Fighter);
    fn set_new_round(&mut self, new_round: Round);
    fn set_attack(
        &mut self,
        attack_index: usize,
        attack: AttackReport<'a>,
        current_health: StatValue,
    );
}

impl<'a> Report<'a> for FullReport<'a> {
    fn new() -> Self {
        FullReport {
            new_round: None,
            attacks: [None, None],
            remaining_healths: [None, None],
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

    fn set_attack(
        &mut self,
        attack_index: usize,
        attack: AttackReport<'a>,
        current_health: StatValue,
    ) {
        self.attacks[attack_index] = Some(attack);
        self.remaining_healths[attack_index] = Some(current_health);
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
    fn set_attack(
        &mut self,
        _attack_index: usize,
        _attack: AttackReport<'a>,
        _current_health: StatValue,
    ) {
    }
}
