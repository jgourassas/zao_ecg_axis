use glu_sys::*;
extern crate glyph_brush;
//use glyph_brush::{ab_glyph::FontArc, BrushAction, BrushError, GlyphBrushBuilder, Section, Text};
use glyph_brush::{ab_glyph::*, *};
extern crate gl;

use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLsizeiptr, GLuint};
use std::f64::consts::PI;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

use fltk::enums::*;
use fltk::{prelude::*, *};

//use fltk::app::fonts::*;

const SIZE_UNIT: f32 = 2.5;
const LEAD_WIDTH: f64 = 0.09;
const OUTER_RADIOUS: f64 = 0.73;
const START_INNER_RADIUS: f64 = 0.05;
const INNER_INCREMENT: f64 = 0.10;
/*
glColor4f(1.0, 0.0, 0.0, 0.0);//red
glColor3f(1.0, 0.5, 0.0);//Orange
glColor3f(0.5, 0.5, 0.5);//Violet
glColor3f(0.0, 0.5, 0.5);//Blue-Green
glColor3f(0.0, 0.5, 1.0);//baby Blue
glColor3f(2.0, 0.5, 1.0);//Lilac
glColor3f(0.1, 0.1, 0.1);//Dark grey
glColor3f(0.1, 0.0, 0.1);//Dark Purple
glColor3f(0.1, 0.1, 0.0);//Bronze
glColor3f(0.0, 0.1, 0.1);//Dark blue
glColor3f(0.0, 0.1, 0.0);//Forest Green
glColor3f(0.1, 0.0, 0.0);//Brown
*/

/////////////////////////
pub fn setup_gl() {
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 0.0);

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();

        glEnable(GL_DEPTH_TEST); // Enable depth buffering
        glDepthFunc(GL_LEQUAL); // Useful for multipass shaders
                                //Set polygon drawing mode for front and back of each triangle
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        glEnable(GL_POINT_SMOOTH);
        glEnable(GL_LINE_SMOOTH);
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST); // Antialias the lines
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    } //unsafe
}
/////////////////////////////
pub fn draw_scene(arrow_angle: &f32) {
    draw_lead_circles();
    set_lead_circles_annotations();
    draw_vector_arrow(arrow_angle);
    draw_outside_circle();
} //draw_scene

////////////////////////////
pub fn draw_lead_circles() {
    unsafe {
        use glu_sys::*;
        let qobj = gluNewQuadric();
        ///////////////////////
        ///////////////////////LEAD NEGATIVE I

        glClear(GL_COLOR_BUFFER_BIT);

        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 0.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 0.0) + LEAD_WIDTH,
            0.0,
            -180.0,
        );
        glPopMatrix();

        ///////////LEAD POSITIVE I
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 0.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 0.0) + LEAD_WIDTH,
            0.0,
            180.0,
        ); //I
        glPopMatrix();

        //////////LEAD POSITIVE II

        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 1.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 1.0) + LEAD_WIDTH,
            60.0,
            -180.0,
        );

        glPopMatrix();
        //////////LEAD NEGATIVE II
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 1.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 1.0) + LEAD_WIDTH,
            60.0,
            180.0,
        ); //II
        glPopMatrix();
        ///////////////// LEAD III POSITIVE //////////////////////////////////////
        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 2.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 2.0) + LEAD_WIDTH,
            120.0,
            -180.0,
        ); //III
        glPopMatrix();
        ///////////////// LEAD III NEGATIVE //////////////////////////////////////
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 2.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 2.0) + LEAD_WIDTH,
            120.0,
            180.0,
        ); //III
        glPopMatrix();
        ////////////////
        //////////////// aVR POSITIVE
        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 3.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 3.0) + LEAD_WIDTH,
            210.0,
            -180.0,
        ); //aVR
        glPopMatrix();
        //////////////////aVR NEGATIVE
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 3.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 3.0) + LEAD_WIDTH,
            210.0,
            180.0,
        ); //aVR
        glPopMatrix();

        /////////////////// AVL POSITIVE////////////////////
        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 4.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 4.0) + LEAD_WIDTH,
            330.0,
            -180.0,
        ); //avL
        glPopMatrix();
        /////////////////// AVL NEGATIVE////////////////////
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 4.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 4.0) + LEAD_WIDTH,
            330.0,
            180.0,
        ); //avL
        glPopMatrix();

        ////////////////////
        ////////////////////
        /////////////////// AVF POSITIVE////////////////////
        glPushMatrix();
        glColor4f(0.0, 0.60, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 5.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 5.0) + LEAD_WIDTH,
            90.0,
            -180.0,
        ); //aVF
        glPopMatrix();
        /////////////////// AVF NEGATIVE////////////////////
        glPushMatrix();
        glColor4f(1.0, 1.0, 1.0, 0.8);
        draw_zao_lead(
            0.0,
            0.0,
            0.0,
            START_INNER_RADIUS + (INNER_INCREMENT * 5.0),
            START_INNER_RADIUS + (INNER_INCREMENT * 5.0) + LEAD_WIDTH,
            90.0,
            180.0,
        ); //aVF
        glPopMatrix();
        /////////////////////END OF LEAD CIRCLES
        gluDeleteQuadric(qobj);
        glFlush();
    } //unsafe
} //draw_lead_circles
  ////////////////////////////
pub fn set_lead_circles_annotations() {
    let mut i: f64 = 0.0;
    let outer_radius: f64 = 0.73 + 0.03;
    //let dejavu = FontArc::try_from_slice(include_bytes!("../../fonts/DejaVuSans.ttf"));
    //  let dejavu = FontRef::try_from_slice(include_bytes!("../../fonts/OpenSans-Light.ttf"));
    // let mut glyph_brush = GlyphBrushBuilder::using_font(dejavu).build();

    unsafe {
        let qobj = gluNewQuadric();
        gluQuadricNormals(qobj, GLU_SMOOTH);

        glPushMatrix();

        glPushMatrix();

        glColor3f(0.0, 0.5, 1.0); //baby Blue
                                  //gl_draw(const char *s, int x, int y)
                                  //void gl_draw(const char *s, int x, int y, int w, int h, Fl_Align)

        //Any point (x,y) on the path of the circle is x = r sin(θ), y = rcos(θ).
        //x = r*cos(a*Pi/180), y = r*sin(a*Pi/180)

        while i <= 24.0 {
            let mut x = outer_radius * ((15.0 * i) * (PI / 180.0)).cos();
            let mut y = outer_radius * ((15.0 * i) * (PI / 180.0)).sin();

            let mut z = 0.0001;

           // println!("x {}, y {}, i {}", x, y, i);

            glPushMatrix();
            glTranslatef(x as f32, y as f32, z as f32);
            gluSphere(qobj, 0.015, 10, 15);
            glPopMatrix();

            ////////////////////////////////
            //write the degrees around the circle
            glPushMatrix(); /////////////1

            glPopMatrix(); //////////////1
            let mut value = 15.0 * i;
            if value > 0.0 && value < 90.1 {
                let value_as_string = value.to_string();
                //gldraw(value_as_string, x as u8, y as u8);
                // let mut buffer = ::rusttyper::RunBuffer::new();

                println!("string value = {}", value_as_string);
            }

            /*

                double value = 1234.66667;
                char result[32];
                sprintf(result, "%08.2d", value);

               char vect_hor_buffer[40];
               glPushMatrix();
               if(15*i>= 0 and 15*i < 90.1 ){
                  sprintf(vect_hor_buffer," %i", -15*i);
                fltk::gldrawtext(vect_hor_buffer,
                                 float(x+0.009),
                                 float(y+0.009),
                                 float(z+0.09) );
               }
                else if (15*i > 90.1  and 15*i < 180.1){
                sprintf(vect_hor_buffer," %i",-15*i);
                fltk::gldrawtext(vect_hor_buffer,
                                 float(x-0.17),
                                 float(y+0.0009),
                                 float(z+0.09) );
                }
                 else if (15*i > 180.1  and 15*i < 270.1 ){
                sprintf(vect_hor_buffer,"+%i",360-(15*i));
                fltk::gldrawtext(vect_hor_buffer,
                                 float(x-0.18),
                                 float(y-0.02),
                                 float(z+0.09) );
                }
                else if (15*i > 270.1  and 15*i < 359.0 ){
                sprintf(vect_hor_buffer,"+%i",360-(15*i));
                fltk::gldrawtext(vect_hor_buffer,
                                 float(x+0.015),
                                 float(y-0.01),
                                 float(z+0.09) );
                }


                         glPopMatrix();
            */
            //////////////////////////////

            i += 1.0;
        }

        /*
              while(i<24){

                fltk::gldrawtext(vect_hor_buffer,
                                     x+0.009,
                                     y+0.009,
                                     z+0.09 );

        */
        glPopMatrix();
      
       
         ///////////////////
         
         glPushMatrix();
        // glColor4f(1.0, 0.0, 0.0, 0.0);//red
         glColor3f(2.0, 0.5, 1.0);//Lilac
         glLineWidth(211.0);
         let mut j = 1.0;
         while j <= 360.0 {
            let mut jx  = OUTER_RADIOUS * ((1.0 * j) * (PI / 180.0)).cos();
           let mut jy  = OUTER_RADIOUS * ((1.0 * j) * (PI / 180.0)).sin();
           let mut z = 0.001;

            glLoadIdentity();
            glPushMatrix();
            glTranslatef(jx as f32, jy as f32, z as f32);
            gluSphere(qobj, 0.004, 10, 15);
            glPopMatrix();

           
           // println!{"j value {}", j};

            j += 1.0;

         }
         glPopMatrix();

         ////////////////////////////
          //gluDeleteQuadric(qobj);
        glFlush();
    } //unsafe
} //set annotations
  /////////////////////////////
  ////////////////
pub fn draw_vector_arrow(arrow_angle: &f32) {
    println!(" Arrow Angle : {}", &arrow_angle.to_string());
    unsafe {
        use glu_sys::*;
        let qobj = gluNewQuadric();
        let inner_radius: f64 = 0.72;
        let outer_radius: f64 = 0.73;

        ///////////draw a sphere at the center Leads
        glPushMatrix();
        //glColor3f(0.5, 0.5, 0.5);//Violet
        glColor3f(1.0, 0.5, 0.0); //Orange
        gluSphere(qobj, 0.02, 10, 15);
        glPopMatrix();

        ///////////////////draw the arrow
        glPushMatrix(); //---------------------

        glTranslatef(0.0, 0.0, 0.0);
        glRotatef(90.0, 0.0, 1.0, 0.0);
        glRotatef(*arrow_angle, 1.0, 0.0, 0.0);

        glPushMatrix();
        glColor4f(1.0, 0.0, 0.0, 0.8); //red
        gluCylinder(qobj, 0.01, 0.01, outer_radius - 0.02, 10, 20);

        glTranslatef(0.0, 0.0, outer_radius as f32 - 0.08);
        gluCylinder(qobj, 0.01 + 0.02, 0.00, 0.1, 10, 20);
        glPopMatrix();

        glPopMatrix(); //---------------------

        gluDeleteQuadric(qobj);
    } //unsafe
} //draw_arrow_vector

//////////////////////////
pub fn draw_outside_circle() {
    unsafe {
        let inner_radius: f64 = 0.72;
        let outer_radius: f64 = 0.73;

        //draw an outside circle
        let qobj = gluNewQuadric();
        glPushMatrix();
        glLineWidth(2.0);
        gluQuadricNormals(qobj, GLU_SMOOTH);
        glColor3f(0.0, 0.5, 1.0); //baby Blue
                                  // glColor3f(1.0, 1.0, 1.0);//Orange
        glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        gluDisk(qobj, inner_radius, outer_radius, 124, 124);
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

        glPopMatrix();

        glFlush();
    } //unsafe
}

////////////////////////////////////

pub fn draw_zao_lead(
    center_x: f32,
    center_y: f32,
    center_z: f32,
    inner_radius: f64,
    outer_radius: f64,
    start_angle: f64,
    end_angle: f64,
) {
    unsafe {
        use glu_sys::*;
        let qobj = gluNewQuadric();
        /////////////////////////
        /////////////////////////////
        glPushMatrix();
        glTranslatef(center_x, center_y, center_z);

        gluQuadricDrawStyle(qobj, GLU_FILL);
        gluQuadricNormals(qobj, GLU_TRUE);
        gluQuadricNormals(qobj, GLU_SMOOTH);
        gluPartialDisk(qobj, inner_radius, outer_radius, 42, 40, start_angle, end_angle);

        glColor3f(0.1, 0.1, 0.1); //Dark grey
        gluPartialDisk(qobj, outer_radius, outer_radius + 0.007, 42, 50, start_angle, end_angle);

        glPopMatrix();

        gluDeleteQuadric(qobj);
    } //unsafe
} //draw_zao_lead
