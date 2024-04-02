use crate::state::{EachFrameImpl, Mode};

use self::dropping::GeneralDropping;

pub mod dropping;

pub struct Weather;

impl Weather {
    pub fn from(mode: Mode) -> impl EachFrameImpl {
        use Mode::*;
        match mode {
            Rain | Snow => GeneralDropping::new(),
            Meteor => todo!(),
            Star => todo!(),
        }
    }
}
