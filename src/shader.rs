extern crate gl;

use cgmath::Array;
use cgmath::Matrix;
use gl::types::*;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;
use std::vec::Vec;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, frag_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // Load vertex shader
        let mut vertex_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("Failed to load vertex shader file: {}", vertex_path));
        let mut vertex_code = String::new();
        vertex_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader file");

        // Load frag shader
        let mut frag_file = File::open(frag_path)
            .unwrap_or_else(|_| panic!("Failed to load frag shader file: {}", frag_path));
        let mut frag_code = String::new();
        frag_file
            .read_to_string(&mut frag_code)
            .expect("Failed to read frag shader file");

        // Create cstring version shader code
        let vertex_code_cstr = match CString::new(vertex_code.as_bytes()) {
            Ok(cstr) => cstr,
            Err(e) => panic!("Failed to create cstr vertex shader code: {:?}", e),
        };
        let frag_code_cstr = match CString::new(frag_code.as_bytes()) {
            Ok(cstr) => cstr,
            Err(e) => panic!("Failed to create cstr frag shader code: {:?}", e),
        };

        unsafe {
            // Create vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code_cstr.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_error(vertex, "VERTEX");

            // Create frag shader
            let frag = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag, 1, &frag_code_cstr.as_ptr(), ptr::null());
            gl::CompileShader(frag);
            shader.check_compile_error(frag, "FRAGMENT");

            // Attach shader to program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, frag);
            gl::LinkProgram(id);
            shader.check_compile_error(id, "PROGRAM");

            // Delete shaders that have been used up
            gl::DeleteShader(vertex);
            gl::DeleteShader(frag);

            shader.id = id;
        }

        return shader;
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_mat(&self, name: &CStr, mat: &cgmath::Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    pub unsafe fn set_vec(&self, name: &CStr, value: &cgmath::Vector3<f32>) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }

    unsafe fn check_compile_error(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::<u8>::with_capacity(2048);
        info_log.set_len(1024 - 1);

        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                println!(
                    "Failed to compile shader: type = {}, log = {}",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            } else {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);

                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );

                    let info_log_string = match String::from_utf8(info_log) {
                        Ok(log) => log,
                        Err(vec) => panic!("Failed to convert to log from buffer: {}", vec),
                    };

                    println!(
                        "Failed to link shader: type = {}, log = {}",
                        type_, info_log_string
                    );
                }
            }
        }
    }
}
