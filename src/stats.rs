pub type Stat = u16;

pub const ATTACK_COSTS: [Stat; 21] = [
    0, 5, 10, 15, 20, 25, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 450, 160, 180,
];

pub const SPEED_COSTS: [Stat; 11] = [0, 10, 20, 35, 50, 70, 90, 110, 135, 160, 190];

pub const ENDURANCE_COSTS: [Stat; 6] = [0, 20, 40, 80, 100, 150];

pub const HEALTH_PER_POINT: Stat = 3;

pub const BASE_HEALTH: Stat = 50;

pub const TOTAL_POINTS: Stat = 200;
