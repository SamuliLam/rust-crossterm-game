use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::Position;
use crate::ui::draw::Draw;
use crate::unit::Point2d;

const CHANGE_IN_SPEED: f64 = 0.1;

#[derive(Default)]
pub enum Direction {
    #[default]
    NORTH,
    NORTHEAST,
    EAST,
    SOUTHEAST,
    SOUTH,
    SOUTHWEST,
    WEST,
    NORTHWEST,
}

impl Direction {
    pub fn as_coordinates(&self) -> Point2d<f64> {
        match self {
            Direction::NORTH => { Point2d { x: 0.0, y: -1.0} }
            Direction::NORTHEAST => { Point2d { x: 0.7061, y: -0.7061} }
            Direction::EAST => { Point2d { x: 1.0, y: 0.0} }
            Direction::SOUTHEAST => { Point2d { x: 0.7061, y: 0.7061 } }
            Direction::SOUTH => { Point2d { x: 0.0, y: 1.0} }
            Direction::SOUTHWEST => { Point2d { x: -0.7061, y: 0.7061 } }
            Direction::WEST => { Point2d { x: -1.0, y: 0.0 } }
            Direction::NORTHWEST => { Point2d { x: -0.7061, y: -0.7061 } }
        }
    }

    pub fn from_xy(x: f64, y:f64) -> Self {
        match (x, y) {
            (0.0, -1.0) => Direction::NORTH,
            (0.7061, -0.7061) => Direction::NORTHEAST,
            (1.0, 0.0) => Direction::EAST,
            (0.7061, 0.7061) => Direction::SOUTHEAST,
            (0.0, 1.0) => Direction::SOUTH,
            (-0.7061, 0.7061) => Direction::SOUTHWEST,
            (-1.0, 0.0) => Direction::WEST,
            (-0.7061, -0.7061) => Direction::NORTHWEST,
            _ => Direction::NORTH
        }
    }
}

#[derive(Default)]
pub struct Player {
    position: Point2d<f64>,
    speed: f64,
    health: u8,
    direction: Direction
}

impl Player {
    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub fn is_alive(&self) -> bool {
        if self.health > 0 {
            true
        } else {
            false
        }
    }

    pub fn take_damage(&mut self, damage: u8) {
        self.health -= damage;
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn accelerate(&mut self) {
        if (self.speed + CHANGE_IN_SPEED) < 1.0 {
            self.speed += CHANGE_IN_SPEED;
        }
    }

    pub fn decelerate(&mut self) {
        if (self.speed - CHANGE_IN_SPEED) > 0.0 {
            self.speed -= CHANGE_IN_SPEED;
        }
    }

    pub fn move_forward(&mut self){
        self.position += self.direction.as_coordinates() * self.speed;
    }

    pub fn forward_position(&self) -> Point2d<u16> {
        (self.position + self.direction.as_coordinates() * self.speed).round().to_u16()
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::NORTH => { self.direction = Direction::NORTHWEST }
            Direction::NORTHEAST => { self.direction = Direction::NORTH }
            Direction::EAST => { self.direction = Direction::NORTHEAST }
            Direction::SOUTHEAST => {self.direction = Direction::EAST}
            Direction::SOUTH => {self.direction = Direction::SOUTHEAST}
            Direction::SOUTHWEST => {self.direction = Direction::SOUTH}
            Direction::WEST => {self.direction = Direction::SOUTHWEST}
            Direction::NORTHWEST => {self.direction = Direction::WEST}
        }
    }


    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::NORTH => { self.direction = Direction::NORTHEAST }
            Direction::NORTHEAST => { self.direction = Direction::EAST }
            Direction::EAST => { self.direction = Direction::SOUTHEAST }
            Direction::SOUTHEAST => { self.direction = Direction::SOUTH }
            Direction::SOUTH => { self.direction = Direction::SOUTHWEST }
            Direction::SOUTHWEST => { self.direction = Direction::WEST }
            Direction::WEST => { self.direction = Direction::NORTHWEST }
            Direction::NORTHWEST => { self.direction = Direction::NORTH }
        }
    }

}

impl Position<f64> for Player {
    fn position(&self) -> Point2d<f64> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<f64>, y_range: Range<f64>)
    where
        f64: PartialOrd + SampleUniform,
        Standard: Distribution<f64>
    {
        let new_position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::NORTH => { write!(f, "↑") }
            Direction::NORTHEAST => {write!(f, "↗")}
            Direction::EAST => {write!(f, "→")}
            Direction::SOUTHEAST => {write!(f, "↘")}
            Direction::SOUTH => {write!(f, "↓")}
            Direction::SOUTHWEST => {write!(f, "↙")}
            Direction::WEST => {write!(f, "←")}
            Direction::NORTHWEST => {write!(f, "↖")}
        }
    }
}

impl Draw<f64> for Player {}

#[derive(Default)]
pub struct PlayerBuilder {
    position: Point2d<f64>,
    speed: f64,
    health: u8,
    direction: Direction
}

impl PlayerBuilder {
    pub fn new() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub fn speed(mut self, s: f64) -> PlayerBuilder {
        self.speed = s;
        self
    }

    pub fn health(mut self, h: u8) -> PlayerBuilder {
        self.health = h;
        self
    }

    pub fn direction(mut self, x: f64, y: f64) -> PlayerBuilder {
        self.direction = Direction::from_xy(x, y);
        self
    }

    pub fn build(self) -> Player {
        Player {
            position: self.position,
            speed: self.speed,
            health: self.health,
            direction: self.direction,
        }
    }

}
