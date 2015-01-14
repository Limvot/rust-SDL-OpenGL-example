extern crate sdl2;
extern crate gl;

fn main() {
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });
    println!("Hello, world!");
}

