extern crate gl;
extern crate sdl2;

use std::mem;
use std::os::raw::c_void;
use std::time::Duration;
use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
type Point3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;



const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main(){
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  
  let window = video_subsystem
        .window("Hello World", WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .build()
        .unwrap();

  // window deal with OpenGL
  let _gl_context = window.gl_create_context().unwrap();
  gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _ );

  // setting size and color 
  unsafe {

    // setting viewport size for cross platform
    let mut dim = [0; 4];
    let dims = &mut dim[0] as *mut gl::types::GLint;
    gl::GetIntegerv(gl::VIEWPORT, dims);
    gl::Viewport(0, 0, *dims.offset(2), *dims.offset(3));

    // if retina, gl::Viewport(0, 0, 800 * 2, 600 * 2);
    gl::ClearColor(0.6, 0.72, 0.86, 1.0);
  }

  // event loop
  let mut event_pump = sdl.event_pump().unwrap();
  'main: loop{
    // exit loop
    for event in event_pump.poll_iter(){
      match event{
        sdl2::event::Event::Quit{ .. } => break 'main,
        _ => {}
      }
    }

    // window color clear with gl::ClearColor(RGBA)
    unsafe{
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    // swap window
    window.gl_swap_window();

    // 60FPS(swap interval)
    ::std::thread::sleep(Duration::new(0,1_000_000_000u32/60));
  }

}