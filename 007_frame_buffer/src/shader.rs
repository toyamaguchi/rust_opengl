use cgmath::Array;
use cgmath::Matrix;
use gl;
use gl::types::*;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Shader {
    pub id: u32,
}

#[allow(dead_code)]
impl Shader {
    #[rustfmt::skip]
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // vertex
        let mut vertex_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", vertex_path));
        let mut fragment_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", fragment_path));
        let mut vertex_code = String::new();

        // fragment
        let mut fragment_code = String::new();
        vertex_file
            .read_to_string(&mut vertex_code)
            .expect("failed to read vertex shader file");
        fragment_file
            .read_to_string(&mut fragment_code)
            .expect("failed to read fragment shader file");

        // create cstring
        let cstr_vertex_code = CString::new(
            vertex_code.as_bytes()).unwrap();
        let cstr_fragment_code = CString::new(
            fragment_code.as_bytes()).unwrap();

        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &cstr_vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &cstr_fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            // delete
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            shader.id = id;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                let info_log_string = match String::from_utf8(info_log) {
                    Ok(log) => log,
                    Err(vec) => panic!("failed to convert to compilation log from buffer: {}", vec),
                };
                println!(
                    "failed to compile shader code: type={}, log={}",
                    type_, info_log_string
                );
            }
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
                    Err(vec) => panic!("failed to convert to link log from buffer: {}", vec),
                };
                println!(
                    "failed to link shader code: type={}, log={}",
                    type_, info_log_string
                );
            }
        }
    }

    #[rustfmt::skip]
    pub fn with_geometry_shader(
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: &str,
    ) -> Self {
        let mut shader = Shader { id: 0 };
        let mut vertex_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", vertex_path));
        let mut fragment_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", fragment_path));
        let mut geometry_file = File::open(geometry_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", geometry_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        let mut geometry_code = String::new();
        vertex_file
            .read_to_string(&mut vertex_code)
            .expect("failed to read vertex shader");
        fragment_file
            .read_to_string(&mut fragment_code)
            .expect("failed to read fragment shader");
        geometry_file
            .read_to_string(&mut geometry_code)
            .expect("failed to read geometry shader");

        let cstr_vertex_code = CString::new(
            vertex_code.as_bytes()).unwrap();
        let cstr_fragment_code = CString::new(
            fragment_code.as_bytes()).unwrap();
        let cstr_geometry_code = CString::new(
            geometry_code.as_bytes()).unwrap();

        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &cstr_vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &cstr_fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // geometry shader
            let geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &cstr_geometry_code.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            shader.check_compile_errors(geometry, "GEOMETRY");

            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::AttachShader(id, geometry);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            // delete
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);
            shader.id = id;
        }

        shader
    }
}
