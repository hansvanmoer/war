/*
 * This file is part of 'The Hundred Years War'.
 * 'The Hundred Years War' is free software: you can redistribute it and/or modify it under the terms of
 * the GNU General Public License as published by the Free Software Foundation,
 * either version 3 of the License, or (at your option) any later version.
 * 'The Hundred Years War' is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or 
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for 
 * more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with 'The Hundred Years War'. If not, see <https://www.gnu.org/licenses/>. 
 *
 */

use gl::types::{GLenum, GLint, GLuint};
use serde::Deserialize;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

///
/// A handle for an OpenGL shader
///
pub struct Shader {
    ///
    /// The OpenGL ID of the shader
    ///
    id: GLuint,
}

impl Shader {
    ///
    /// Loads a new shader from a path
    ///
    pub fn load(path: &Path, kind: ShaderKind) -> Result<Shader, Error> {
	let mut buffer = String::new();
	File::open(path)?.read_to_string(&mut buffer)?;
	Shader::from_str(&buffer, kind)
    }
    
    ///
    /// Compiles a new shader from source
    ///
    pub fn from_str(source: &str, kind: ShaderKind) -> Result<Shader, Error> {
	let source = CString::new(source).map_err(|_| Error::BadSource)?;
	let id = unsafe {
	    let id = gl::CreateShader(kind.type_enum());
	    if id == 0 {
		Err(Error::CreateShader)
	    } else {
		Ok(id)
	    }
	}?;
	unsafe {
	    gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
	    gl::CompileShader(id);
	}
	let mut status: GLint = 1;
	unsafe {
	    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
	}
	if status == 0 {
	    Err(Shader::create_compile_error(id))
	} else {
	    Ok(Shader {
		id,
	    })
	}
    }

    ///
    /// Creates the compilation error
    ///
    fn create_compile_error(id: GLuint) -> Error {
	let mut length: gl::types::GLint = 1;
	unsafe {
	    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
	}
	let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
	buffer.extend([b' '].iter().cycle().take(length as usize));
	let buffer: CString = unsafe {
	    CString::from_vec_unchecked(buffer)
	};
	unsafe {
	    gl::GetShaderInfoLog(id, length, std::ptr::null_mut(), buffer.as_ptr() as * mut gl::types::GLchar);
	}
	Error::Compile(buffer.to_string_lossy().into_owned())
    }

    ///
    /// Attaches the shader to a program
    /// The caller is supposed to keep the result in scope until it has to be detached
    ///
    pub fn attach(self, program_id: GLuint) -> AttachedShader {
	unsafe {
	    gl::AttachShader(program_id, self.id);
	}
	AttachedShader {
	    program_id,
	    shader_id: self.id,
	}
    }
    
}

impl Drop for Shader {
    ///
    /// Releases the resources associated with the shader
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteShader(self.id);
	}
    }
}

pub struct AttachedShader {
    ///
    /// The program ID
    ///
    program_id: GLuint,
    ///
    /// The shader ID
    ///
    shader_id: GLuint,
}

impl AttachedShader {
    ///
    /// Detaches the shader
    ///
    pub fn detach(self) -> Shader {
	unsafe {
	    gl::DetachShader(self.program_id, self.shader_id);
	}
	Shader {
	    id: self.shader_id,
	}
    }
}

///
///  
///
impl Drop for AttachedShader {
    ///
    /// Releases the resources associated with the shader
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DetachShader(self.program_id, self.shader_id);
	    gl::DeleteShader(self.shader_id);
	}
    }
}

///
/// The type of shader
///
#[derive(Deserialize)]
pub enum ShaderKind {
    ///
    /// A vertex shader
    ///
    Vertex,
}

impl ShaderKind {
    ///
    /// returns the shader type enum
    ///
    fn type_enum(&self) -> GLenum {
	match self {
	    ShaderKind::Vertex => gl::VERTEX_SHADER,
	}
    }
}

///
/// Errors that can occur compiling a shader
///
#[derive(Debug)]
pub enum Error {
    ///
    /// IO error while loading the shader
    ///
    IO(std::io::Error),
    ///
    /// Source code is not a C string
    ///
    BadSource,
    ///
    /// Create shader method failed
    ///
    CreateShader,
    ///
    /// Compilation error
    ///
    Compile(String),
}

impl From<std::io::Error> for Error {
    ///
    /// Creates an error from an IO error
    ///
    fn from(e: std::io::Error) -> Error {
	Error::IO(e)
    }
}
