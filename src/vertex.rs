use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

use core::ffi::c_void;
use std::mem;

pub struct Vertex {
    vao: u32,
    vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        attr_types: std::vec::Vec<GLenum>,
        attr_sizes: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        let mut vao = 0;
        let mut vbo = 0;

        // Use unsafe block to use OpenGL functions
        unsafe {
            // Generate vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // Bind array buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Transfer vertex data 1st time
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;

            for i in 0..attr_types.len() {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    attr_sizes[i],
                    attr_types[i],
                    gl::FALSE,
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attr_sizes[i] as usize;
            }

            // Unbind buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao,
            vbo,
            vertex_num,
        }
    }

    pub fn draw(&self) {
        // Use unsafe block to use OpenGL functions
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            gl::BindVertexArray(0);
        }
    }
}
