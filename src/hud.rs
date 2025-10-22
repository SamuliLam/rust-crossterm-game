use std::fmt::{write, Display, Formatter};
use std::ops::Range;
use position_derive::Position;
use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::Position;
use crate::ui::draw::Draw;
use crate::unit::{Player, Point2d};

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    position: Point2d<u16>,
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
        Self {
            score,
            player,
            position: Point2d { x: 0, y: y_position }
        }
    }
}

impl<'a> Display for Hud<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score: {} Health: {} Speed: {}", self.score, self.player.health(), self.player.speed())
    }
}

impl<'a> Position<u16> for Hud<'a> {
    fn position(&self) -> Point2d<u16> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<u16>, y_range: Range<u16>)
    where
        u16: PartialOrd + SampleUniform,
        Standard: Distribution<u16>
    {
        let new_position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}

impl<'a> Draw<u16> for Hud<'a> {}
