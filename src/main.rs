extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate sdl2_net;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi;
use collections::vec;

static VERTEX_DATA: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
];

static VS_SRC: &'static str =
    "#version 150\n\
    in vec3 position;\n\
    void main() {\n\
        gl_Position = vec4(position, 1.0);\n\
    }";

static FS_SRC: &'static str =
    "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
        out_color = vec4(1.0, 0.5, 0.5, 1.0);\n\
    }";

fn compile_shader(src: &str, ty:GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        gl::ShaderSource(shader, 1, &ffi::CString::from_slice(src.as_bytes()).as_ptr(), ptr::null());
        gl::CompileShader(shader);
        // Get the status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        
        // If there was an error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity((len-1) as usize); // -1 to skip trailing null
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).unwrap());
        }
    }
    shader
}

fn link_program(vertexShader: GLuint, fragmentShader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertexShader);
        gl::AttachShader(program, fragmentShader);
        gl::LinkProgram(program);
        // Link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity((len-1) as usize); // -1 to skip trailing null
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).unwrap());
        }
        program
    }
}
        

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    // Some SDL2_net tests

    unsafe { sdl2_net::ffi::SDLNet_Init(); }
    //return;

    //

    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCoreProfile as i32);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    let window = match Window::new("rust-sdl2: Video", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("faid to create window: {}", err)
    };

    // MUST ASSIGN RESULT THIS TO A VARIABLE
    // Otherwise, it gets deleted or is optimized out or something
    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });

    let vertexShader = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fragmentShader = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vertexShader, fragmentShader);

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        // create vertex array obj
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // create vertex buffer obj
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);
        gl::UseProgram(program);
        gl::GetAttribLocation(program, ffi::CString::from_slice("out_color".as_bytes()).as_ptr());

        // specify location of vertex data
        let pos_attr = gl::GetAttribLocation(program, ffi::CString::from_slice("position".as_bytes()).as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 3, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }

    loop {
        match poll_event() {
            Quit(_) => break,
            KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
    }
    // clean up
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fragmentShader);
        gl::DeleteShader(vertexShader);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
    sdl2::quit();
}

