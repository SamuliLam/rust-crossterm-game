use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::Position;
use crate::ui::draw::Draw;
use crate::unit::{Enemy, Point2d};

#[derive(Default)]
pub struct Collectible {
    position: Point2d<u16>,
}

impl Position<u16> for Collectible {
    fn position(&self) -> Point2d<u16> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position
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


impl Display for Collectible {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "❤️")
    }
}

impl Draw<u16> for Collectible {}