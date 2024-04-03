use crate::{cli::Args, state::{EachFrameImpl, Mode}, widget::AsWeatherWidget};

use self::dropping::GeneralDropping;

pub mod dropping;

pub struct Weather;

impl Weather {
    pub fn from(args: Args) -> impl EachFrameImpl + AsWeatherWidget {
        use Mode::*;
        match args.mode {
            Rain | Snow => GeneralDropping::new(args),
            Meteor => todo!(),
            Star => todo!(),
        }
    }
}

