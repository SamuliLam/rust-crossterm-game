use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::Position;
use crate::ui::draw::Draw;
use crate::unit::{Player, Point2d};

#[derive(Default)]
pub struct Enemy {
    position: Point2d<f64>,
    speed: f64,
}

impl Enemy {
    pub fn with_speed(s: f64) -> Self {
        Enemy {
            position: Point2d {x: 0.0, y:0.0},
            speed: s,
        }
    }

    pub fn move_towards_player(&mut self, player: &Player) {
        let direction = (player.position() - self.position).normalize();
        self.position += direction * self.speed;
    }
}


impl Display for Enemy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "👾")
    }
}

impl Draw<f64> for Enemy {}

impl Position<f64> for Enemy {
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