use crate::{
    cli::Args,
    state::{EachFrameImpl, Mode},
    weather::dropping::GeneralDropping, widget::AsWeatherWidget,
};

pub mod dropping;

pub struct Weather;

impl Weather {
    pub fn from(args: Args) -> impl EachFrameImpl + AsWeatherWidget {
        use Mode::*;
        match args.mode {
            Rain | Snow => GeneralDropping::new(args),
            _ => panic!("has not been implemented yet for this mode"),
        }
    }
}
