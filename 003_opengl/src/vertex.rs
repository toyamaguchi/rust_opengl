use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // create vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // bind buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    attribute_size_vec[i],
                    attribute_type_vec[i],
                    gl::FALSE,
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            gl::BindVertexArray(0);
        }
    }
}
