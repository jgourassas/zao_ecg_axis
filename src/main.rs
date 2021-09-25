#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use fltk::*;
use fltk::{prelude::*, *};
use fltk::dialog::message;
use fltk::{frame::*,  frame::Frame, window::*, button::Button};
use fltk::enums::Color;


extern crate gl;
use gl::types::*;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use std::cell::RefCell;
use std::rc::Rc;

use std::{error::Error, fmt, panic, thread, time};

mod graphics;
use graphics::draw::{
    draw_scene,
    draw_arrow,
    setup_gl,
};
mod controls;
use controls::button::MyButton;
use controls::slider::MySlider;




const MARGIN_TOP: i32 = 50;
const MARGIN_BOTTOM: i32 = 100;
const MARGIN_RIGHT: i32 = 220;
const MARGIN_LEFT: i32 = 10;

const FLTK_WINDOW_WIDTH: i32 = 1320 - MARGIN_LEFT - MARGIN_RIGHT;
const FLTK_WINDOW_HEIGHT: i32 = 1320 - MARGIN_TOP - MARGIN_BOTTOM;

const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 50;
const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 100;

const FRAME_INFO_X: i32 = 10;
const FRAME_INFO_Y: i32 = GL_WINDOW_HEIGHT + 10   ;
const FRAME_INFO_WIDTH: i32 = GL_WINDOW_WIDTH;
const FRAME_INFO_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - GL_WINDOW_HEIGHT;

const SLIDER_X: i32 = FRAME_INFO_X  + 400;
const SLIDER_Y: i32 = FRAME_INFO_Y + 30; 
const SLIDER_BOUNDS: [i32; 2] = [-180, 180];
const SLIDER_TITLE: &str = "Move Arrow";

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ArrowRotate,
}

pub enum EventMessage {
    PushEvent,
}


pub fn main() {

    panic::set_hook(Box::new(|panic_info| {
        message(200, 200, &panic_info.to_string());
    }));

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(255,255,255);
        app::set_visible_focus(false);

    let mut fltk_wind = Window::new(
        100,
        100,
        FLTK_WINDOW_WIDTH,
        FLTK_WINDOW_HEIGHT,
        "Zao ECG Axis.  By John Gkourasas (J. Gourassas)",
    );

    

    //let mut gl_wind = window::GlutWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!" );
    let mut gl_wind = window::GlWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!" );
    gl_wind.set_mode(enums::Mode::Opengl3);

    gl_wind.make_resizable(true);


    let arrow_angle = Rc::from(RefCell::from(0.0));
    let arrow_angle_c = arrow_angle.clone();

      

    gl_wind.draw(move |_| 
        draw_scene(&arrow_angle_c.borrow())   );


   // gl_wind.draw(Box::new(move || {
   //    setup_gl();
   //   //draw_scene(&rao_lao_c.borrow(), &cr_ca_c.borrow())
   //    draw_scene()
   //}));


   let mut frame_info_view = Frame::new(
    FRAME_INFO_X,
    FRAME_INFO_Y,
    FRAME_INFO_WIDTH,
    FRAME_INFO_HEIGHT,
    "",
);
//frame_info_view.set_color(Color::from_rgb(222, 235, 247)); //white
    frame_info_view.set_frame(enums::FrameType::RFlatBox);
    frame_info_view.set_color(Color::from_rgb(204, 255, 255));
    frame_info_view.set_label_size(20);
    //frame_info_view.set_label("Axis Data");


    let mut but_quit =
    MyButton::new(FRAME_INFO_X + 20, FRAME_INFO_Y+20,  70, 40, "Quit➤");
    but_quit.set_color(Color::from_rgb(255, 0, 0));
    but_quit.set_frame(enums::FrameType::OFlatFrame);

   
    let mut slider = MySlider::new(
        SLIDER_X,
        SLIDER_Y,
        350,
        35,
        SLIDER_TITLE,
        SLIDER_BOUNDS,
    );

    let slider_c = slider.clone();

    let mut frame_slider =
        Frame::new(SLIDER_X - 60, SLIDER_Y, 400, 140, "Frame For Slider");

    frame_slider.set_color(Color::from_rgb(252, 141, 89)); //orange like
    frame_slider.set_label_size(20);
    frame_slider.set_label("Frame Slider");

    gl_wind.end();
    gl_wind.show();


    fltk_wind.make_resizable(true);
    fltk_wind.end();
    fltk_wind.show();

    let (s, r) = app::channel::<(Message, f32)>();
  
   slider.set_callback(move |_| { 
       let arrow_angle = slider_c.value() as f32;
        let msg = (Message::ArrowRotate, arrow_angle);
        s.send(msg);
    });


    but_quit.set_callback(|_| cb_quit());

    while app.wait() {
        if let Some(msg) = r.recv() {
    match msg {
                (Message::ArrowRotate, angle) => {
                    *arrow_angle.borrow_mut() = angle;
                    frame_slider.set_label(&(angle).to_string());
                    //println!("{}", &angle.to_string());

                    //gl_wind.redraw();
                }
                //_ => continue,
                _ => println!(" Message End"),
            } //match msg

        }//if
    }//while

}//main


fn cb_quit() {
    println!("Enjoy. Tnx ");
    std::process::exit(0x0100);
} //cb_quit
  /***********************************************************/
