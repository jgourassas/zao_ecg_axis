#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use fltk::{
    app, app::*, button::Button, dialog::message, draw, enums::Color, enums::*, frame::Frame,
    frame::*, prelude::*, window::*,
};

use fltk::enums;
use fltk::window;

use std::cell::RefCell;
use std::rc::Rc;

use std::ffi::CString;
use std::ptr;
use std::str;
use std::{error::Error, fmt, mem, panic, thread, time};


extern crate gl;
use gl::types::*;
//extern crate glium;
//extern crate glium_text;

//use glu_sys::*;
mod graphics;
use graphics::draw::{
    draw_lead_circles, draw_outside_circle, draw_scene, 
    draw_vector_arrow, setup_gl, draw_lead_names
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

const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 60;
const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 120;

const FRAME_INFO_X: i32 = 10;
const FRAME_INFO_Y: i32 = GL_WINDOW_HEIGHT - 5;
//const FRAME_INFO_WIDTH: i32 = GL_WINDOW_WIDTH;
//const FRAME_INFO_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - GL_WINDOW_HEIGHT;

const SLIDER_X: i32 = FRAME_INFO_X + 350;
const SLIDER_Y: i32 = FRAME_INFO_Y + 30;
const SLIDER_BOUNDS: [i32; 2] = [-180, 180];
const SLIDER_TITLE: &str = "Move Arrow";

//
//const GL_WIND:  window::GlWindow =
  //      window::GlWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!");
#[derive(Debug, Clone, Copy)]

pub enum Message {
    ArrowRotate,
}

pub enum EventMessage {
    PushEvent,
}

pub fn main() {
    panic::set_hook(Box::new(|panic_info| {
        fltk::dialog::message(200, 200, &panic_info.to_string());
    }));

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    //app::background(255,255,255);
    //app::set_visible_focus(false);

    let mut fltk_wind = window::Window::new(
        100,
        100,
        FLTK_WINDOW_WIDTH,
        FLTK_WINDOW_HEIGHT,
        "Zao ECG Axis.  By John Gkourasas (J. Gourassas)",
    );

    let mut slider = MySlider::new(SLIDER_X, SLIDER_Y, 350, 35, SLIDER_TITLE, SLIDER_BOUNDS);

    let slider_c = slider.clone();

    let mut frame_slider = Frame::new(SLIDER_X - 70, SLIDER_Y, 70, 50, "");
    frame_slider.set_color(enums::Color::from_rgb(252, 141, 89)); //orange like
    frame_slider.set_label_size(20);
    // frame_slider.set_label("Frame Slider");

    let mut but_quit = MyButton::new(FRAME_INFO_X + 20, FRAME_INFO_Y + 20, 70, 40, "Quit➤");
    but_quit.set_color(enums::Color::from_rgb(255, 0, 0));
    but_quit.set_frame(fltk::enums::FrameType::OFlatFrame);

    
    let mut gl_wind: window::GlWindow =
        window::GlWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!");

   // let mut gl_wind: Fl_Glut_Window =
   //     window::GlWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!");
    
        ///////////jgour///
    //let system = glium_text::TextSystem::new(&gl_wind);

    ///////////////////
        // gl_wind.set_mode(enums::Mode::Opengl3);//Is flickring
    gl_wind.make_resizable(true);

    let arrow_angle = Rc::from(RefCell::from(0.0));
    let arrow_angle_c = arrow_angle.clone();
    
    
    gl_wind.draw(move |_| {
               draw_scene(&arrow_angle_c.borrow());
               //draw_lead_names(&gl_wind_c.borrow());


              });

    let _gl = gl::load_with(|s| gl_wind.get_proc_address(s) as *const std::os::raw::c_void);
    gl_wind.end();
    gl_wind.show();

    fltk_wind.make_resizable(true);
    fltk_wind.end();
    fltk_wind.show();

    let (s, r) = app::channel::<(Message, f32)>();

    gl_wind.handle(|t, ev| {
        if ev == Event::Push && app::event_clicks() {
            //  t.top_window().unwrap().hide();
            println!("Key pressed in window ");
            true
        } else {
            false
        }
    });

    /*
        gl_wind.handle(|t, ev| {
            if ev == Event::Push && app::event_clicks() {
              //  t.top_window().unwrap().hide();
                println!("Key pressed in window ");
                true
            } else {
                false
            }
        });
    nt MyWindow::handle(int event) {
      switch(event) {
      case FL_PUSH:
        ... mouse down event ...
        ... position in Fl::event_x() and Fl::event_y()
        return 1;
      case FL_DRAG:
        ... mouse moved while down event ...
        return 1;
      case FL_RELEASE:
        ... mouse up event ...
        return 1;
      case FL_FOCUS :
      case FL_UNFOCUS :
        ... Return 1 if you want keyboard events, 0 otherwise
        return 1;
      case FL_KEYBOARD:
        ... keypress, key is in Fl::event_key(), ascii in Fl::event_text()
        ... Return 1 if you understand/use the keyboard event, 0 otherwise...
        return 1;
      case FL_SHORTCUT:
        ... shortcut, key is in Fl::event_key(), ascii in Fl::event_text()
        ... Return 1 if you understand/use the shortcut event, 0 otherwise...
        return 1;
      default:
        // pass other events to the base class...
        return Fl_Gl_Window::handle(event);
      }
    }
    */
    but_quit.set_callback(|_| cb_quit());

    slider.set_callback(move |_| {
        let arrow_angle = slider_c.value() as f32;
        let msg = (Message::ArrowRotate, arrow_angle);
        s.send(msg);
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                (Message::ArrowRotate, angle) => {
                    *arrow_angle.borrow_mut() = angle;
                    frame_slider.set_label(&(angle).to_string());
                    gl_wind.redraw();
                    //gl_wind.swap_buffers();
                    //app::sleep(0.026);
                    //app::awake();
                }
                //_ => continue,
                _ => println!(" Message End"),
            } //match msg
        } //if
    } //while
} //main

fn cb_quit() {
    println!("Enjoy. Tnx ");
    std::process::exit(0x0100);
} //cb_quit
  /***********************************************************/
