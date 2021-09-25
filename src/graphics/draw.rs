use glu_sys::*;

extern crate gl;
use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLsizeiptr, GLuint };
use std::f64::consts::PI;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const SIZE_UNIT: f32 = 2.5;
const LEAD_WIDTH: f64 = 0.09;
const OUTER_RADIOUS: f64 = 0.85;
const START_INNER_RADIUS: f64 = 0.05;
const INNER_INCREMENT: f64 = 0.10;
/*
glColor4f(1.0f, 0.0f, 0.0f, 0.0f);//red
lColor3f(1.0f, 0.5f, 0.0f);//Orange
glColor3f(0.5f, 0.5f, 0.5f);//Violet
glColor3f(0.0f, 0.5f, 0.5f);//Blue-Green

glColor3f(0.0f, 0.5f, 1.0f);//baby Blue

glColor3f(2.0f, 0.5f, 1.0f);//Lilac

glColor3f(0.1f, 0.1f, 0.1f);//Dark grey

glColor3f(0.1f, 0.0f, 0.1f);//Dark Purple

glColor3f(0.1f, 0.1f, 0.0f);//Bronze

glColor3f(0.0f, 0.1f, 0.1f);//Dark blue

glColor3f(0.0f, 0.1f, 0.0f);//Forest Green

glColor3f(0.1f, 0.0f, 0.0f);//Brown
*/

pub fn setup_gl() {
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 0.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();

        glEnable(GL_DEPTH_TEST); // Enable depth buffering
        glDepthFunc(GL_LEQUAL); // Useful for multipass shaders
                                // Set polygon drawing mode for front and back of each triangle
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        // glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        // glPolygonMode(GL_FRONT_AND_BACK, GL_POINT);
        // Disable backface culling to render both sides of polygons
        // glDisable(GL_CULL_FACE);

        // The following commands should induce OpenGL to create round points and
        //  antialias points and lines.  (This is implementation dependent unfortunately.)
        glEnable(GL_POINT_SMOOTH);
        glEnable(GL_LINE_SMOOTH);
        glHint(GL_POINT_SMOOTH_HINT, GL_NICEST); // Make round points, not square points
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST); // Antialias the lines
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    } //unsafe
}
/////////////////////////////

pub fn draw_scene(rotate_arrow: &f32) {
    setup_gl();
    ///////////////////////////
    unsafe {
        use glu_sys::*;
        let qobj = gluNewQuadric();
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
        ); //I
        glPopMatrix();

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
        ///////////LEAD II

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
        ); //II

        glPopMatrix();

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

        ///////////////// III //////////////////////////////////////
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
        //////////////// aVR
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

        ////////////////////
        /////////////////// AVL ////////////////////
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
//////////////////// avf

glPushMatrix();
glColor4f(0.0, 0.60, 1.0, 0.8);
draw_zao_lead(0.0,0.0,0.0, 
                            START_INNER_RADIUS+(INNER_INCREMENT*5.0), 
                      START_INNER_RADIUS+(INNER_INCREMENT*5.0)+LEAD_WIDTH, 
     90.0, -180.0); //aVF
glPopMatrix();

glPushMatrix();
glColor4f(1.0, 1.0, 1.0, 0.8);
draw_zao_lead(0.0,0.0,0.0, 
                     START_INNER_RADIUS+(INNER_INCREMENT*5.0), 
                      START_INNER_RADIUS+(INNER_INCREMENT*5.0)+LEAD_WIDTH, 
     90.0, 180.0); //aVF
glPopMatrix();

//////////////////////////
glPushMatrix();
   glColor3f(0.0,1.0,0.0);//green
 	glTranslatef(0.0,-0.85,0.0);
 	glRotatef(90.0, 1.0, 0.0, 0.0);
        //DRAW THE X RAY GENERETOR
	gluCylinder(qobj,
		  0.03,
		  0.2,
		  0.1,
		  20,
		  30 );
  glPopMatrix();
//
// //THE BOOTOM OF THE GENERATOR
  glPushMatrix();
        glColor3f(0.0,1.0,0.0);
 	glTranslatef(0.0,-0.95,0.0);
 	glRotatef(90.0, 1.0, 0.0, 0.0);
  //the generetor
	gluCylinder(qobj,
		  0.2,
		  0.2,
		  0.1,
		  20,
		  30 );
  glPopMatrix();
  //draw a sphere in the center 
  glPushMatrix();
  
  glColor4f(1.0, 0.0, 0.0, 1.0);
    glTranslatef(0.0,0.0,0.0);
    glRotatef(90.0, 1.0, 0.0, 0.0);
   // glRotatef(*rotate_arrow, 0.0, 0.0, 1.0);
    gluSphere(qobj,
		  0.02,
		  10,
		  15 );
  glPopMatrix();
  gluDeleteQuadric(qobj);

  zao_frontal_circle_indicator(*rotate_arrow);
  glFlush();
////////////////////////////
    }

    /////////////////////////
} //draw

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

        glPushMatrix();
        glTranslatef(center_x, center_y, center_z);
         
        gluQuadricDrawStyle(qobj, GLU_FILL);
        gluQuadricNormals(qobj, GLU_TRUE);
        gluPartialDisk(
            qobj,
            inner_radius,
            outer_radius,
            42,
            40,
            start_angle,
            end_angle,
        );
        glPopMatrix();

        glPushMatrix();
        glColor3f(0.1, 0.1, 0.1); //Dark grey
        gluPartialDisk(
            qobj,
            outer_radius,
            outer_radius + 0.007,
            42,
            50,
            start_angle,
            end_angle,
        );
        glPopMatrix();

        gluDeleteQuadric(qobj);

    } //unsafe
} //draw_zao_lead


pub fn zao_frontal_circle_indicator(rotate_arrow: f32){
    unsafe{
    let qobj = gluNewQuadric();
     gluQuadricDrawStyle(qobj, GLU_FILL);
     gluQuadricTexture(qobj,GL_TRUE as u8);
     
     gluQuadricNormals(qobj, GLU_TRUE);
     
     glColor4f(0.0, 1.0, 0.0, 1.0);
     let inner_radius: f64 = 0.72;
     let outer_radius: f64 = 0.73;
        glPushMatrix();
           glColor4f(1.0,1.0,1.0,1.0);
           glTranslatef(0.0, 0.0, 0.0);
           gluPartialDisk(qobj, inner_radius, outer_radius, 142, 140, 0.0, 360.0);
        glPopMatrix();
       
        glPushMatrix();

        //glColor4f(1.0,1.0,1.0,1.0);
        //glColor4f(0.5, 0.5, 0.5, 1.0);//Violet
        glColor3f(0.0, 0.5, 1.0);//baby Blue
        let mut i = 0.0;
        while  i < 24.0 {
             let x: f64 = ((15.0*i)*(PI / 180.0)).cos() * (outer_radius+0.03);
             let y: f64 = ((15.0*i)*(PI / 180.0)).sin() * (outer_radius+0.03);
             let z: f64 = 0.0001;
             glPushMatrix();
               glTranslatef(x as f32,y as f32, z as f32);
               gluSphere(qobj, 0.015, 10, 15 );
              // gluSphere(quad: *mut GLUquadric, radius: GLdouble, slices: GLint, stacks: GLint)   
              glPopMatrix();
          
              ///////////////////////////
          //write the degrees around the circle
          // char vect_hor_buffer[40];
          let vect_hor_buffer: Vec<String>;
          

    glPushMatrix();

   // use fltk_sys::*;
         //glsetfont(HelveticaBoldItalic, 18 );
        
           if 15.0*i>= 0.9 &&  15.0*i < 90.1 {
            

          //  sprintf(vect_hor_buffer," %i", -15*i);
              // fltk::gldrawtext("OK",
              //              x+0.009, 
              //              y+0.009,
              //              z+0.09 );    

           //  sprintf(vect_hor_buffer," %i", -15*i);

            //fltk::gldrawtext(vect_hor_buffer,
            //                 float(x+0.009), 
            //                 float(y+0.009),
           //                  float(z+0.09) );
           }
           /*
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

*/
                     glPopMatrix();


              /////////////////////////////
          i+=1.0;

        }
        draw_arrow(&rotate_arrow);

//         glPopMatrix();
  
    }//unsafe

}//zao_frontal_circle_indicator

pub fn draw_arrow(rotate_arrow: &f32){
    unsafe{
        //let inner_radius: f64 = 0.72;
        let outer_radius: f64 = 0.73;

    let qobj = gluNewQuadric();
     gluQuadricDrawStyle(qobj, GLU_FILL);
     gluQuadricTexture(qobj,GL_TRUE as u8);
     
     gluQuadricNormals(qobj, GLU_TRUE);
     
    glPushMatrix();
    glColor4f(1.0, 0.0, 0.0, 1.0);//red
    glRotatef(90.0, 0.0, 1.0, 0.0);
    
    //println!("{}", &rotate_arrow.to_string());
    //println!("rotate_angle = {}",  &rotate_arrow.to_string());
    glRotatef(*rotate_arrow, 1.0, 0.0, 0.0);
    gluCylinder(qobj,
     0.01,
     0.01,
     outer_radius - 0.02,
     10,
     20 );
     glTranslatef(0.0, 0.0, outer_radius  as f32  - 0.08);
     gluCylinder(qobj,
         0.01 + 0.02,
         0.00,
         0.1,
         10,
         20 );

   glPopMatrix();
     }//unsafe

}