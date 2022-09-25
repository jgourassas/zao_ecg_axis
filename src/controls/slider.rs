use fltk::enums::Color;
use fltk::*;
use fltk::{button::Button, frame::Frame, frame::*, window::*};
use fltk::{prelude::*, valuator::*, *};

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]

pub struct MySlider {
    sl: valuator::HorNiceSlider,
} //struct

impl MySlider {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str, bounds: [i32; 2]) -> MySlider {
        let mut sl = valuator::HorNiceSlider::new(
            x,
            y,
            w,
            h,
            "Rotate Arrow - Polarity circles are From within to outword I,II,III,aVR,aVL,aVF
             - Positive is White - Negative is Blue ",
        );
        sl.set_bounds(bounds[0] as f64, bounds[1] as f64);
        sl.set_frame(enums::FrameType::RoundUpBox);
        sl.set_step(1.0, 1);
        sl.set_callback(move |b| b.parent().unwrap().hide());
        sl.set_color(Color::from_rgb(158, 188, 218));
        //sl.set_color(Color::from_rgb(189, 201, 225));
        sl.set_label_size(18);

        MySlider { sl } //MySlider
    } //new
} //impl MySlider

impl Deref for MySlider {
    type Target = HorNiceSlider;

    fn deref(&self) -> &Self::Target {
        &self.sl
    }
}

impl DerefMut for MySlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sl
    }
}
