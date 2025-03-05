use anyhow::{Context, Result};
use std::ffi::CString;
use log::{info, error};
use gl::*;

pub struct Shader {
    pub program: u32,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Shader> {
        info!("Creating shader program...");
        let program = unsafe { gl::CreateProgram() };
        if program == 0 {
            error!("Failed to create shader program");
            return Err(anyhow::anyhow!("Failed to create shader program"));
        }
        info!("Created shader program with ID: {}", program);

        info!("Compiling vertex shader...");
        info!("Vertex shader source:\n{}", vertex_source);
        let vertex_shader = compile_shader(vertex_source, gl::VERTEX_SHADER)
            .context("Failed to compile vertex shader")?;
        info!("Vertex shader compiled successfully with ID: {}", vertex_shader);
        
        info!("Compiling fragment shader...");
        info!("Fragment shader source:\n{}", fragment_source);
        let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER)
            .context("Failed to compile fragment shader")?;
        info!("Fragment shader compiled successfully with ID: {}", fragment_shader);
        
        info!("Attaching shaders to program...");
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
        }

        info!("Linking shader program...");
        unsafe {
            gl::LinkProgram(program);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring(len as usize);
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
                let error_msg = error.to_string_lossy().to_string();
                error!("Shader linking error: {}", error_msg);
                error!("Vertex shader source:\n{}", vertex_source);
                error!("Fragment shader source:\n{}", fragment_source);
                
                // 清理资源
                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
                gl::DeleteProgram(program);
                
                return Err(anyhow::anyhow!("Shader linking error: {}", error_msg));
            }

            info!("Shader program linked successfully");

            // 清理着色器对象
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_vec3(&self, name: &str, value: &[f32; 3]) {
        let name_c_str = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name_c_str.as_ptr());
            if location == -1 {
                error!("Failed to get uniform location for '{}' in shader program {}", name, self.program);
                return;
            }
            gl::Uniform3fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_vec4(&self, name: &str, value: &[f32; 4]) {
        let name_c_str = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name_c_str.as_ptr());
            if location == -1 {
                error!("Failed to get uniform location for '{}' in shader program {}", name, self.program);
                return;
            }
            gl::Uniform4fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_mat4(&self, name: &str, value: &[f32; 16]) {
        let name_c_str = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name_c_str.as_ptr());
            if location == -1 {
                error!("Failed to get uniform location for '{}' in shader program {}", name, self.program);
                return;
            }
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let name_c_str = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name_c_str.as_ptr());
            if location == -1 {
                error!("Failed to get uniform location for '{}' in shader program {}", name, self.program);
                return;
            }
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let name_c_str = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name_c_str.as_ptr());
            if location == -1 {
                error!("Failed to get uniform location for '{}' in shader program {}", name, self.program);
                return;
            }
            gl::Uniform1i(location, value);
        }
    }
}

fn compile_shader(source: &str, shader_type: u32) -> Result<u32> {
    info!("Creating shader...");
    let shader = unsafe { gl::CreateShader(shader_type) };
    if shader == 0 {
        error!("Failed to create shader object");
        return Err(anyhow::anyhow!("Failed to create shader object"));
    }
    info!("Created shader with ID: {}", shader);

    info!("Setting shader source...");
    let c_str = CString::new(source.as_bytes()).context("Shader source contains null byte")?;
    
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        
        info!("Compiling shader...");
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let error = create_whitespace_cstring(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
            let error_msg = error.to_string_lossy().to_string();
            error!("Shader compilation error: {}", error_msg);
            error!("Shader source:\n{}", source);
            gl::DeleteShader(shader);
            return Err(anyhow::anyhow!("Shader compilation error: {}", error_msg));
        }

        info!("Shader compiled successfully");
    }

    Ok(shader)
}

fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
} 