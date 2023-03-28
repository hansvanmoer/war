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

use super::shader::{AttachedShader, Shader};

use gl::types::GLuint;

///
/// An OpenGL  program
///
pub struct Program {
    ///
    /// The OpenGL ID fo the program
    ///
    id: GLuint,
}

impl Drop for Program {
    ///
    /// Releases the OpenGL resources linked to this program
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteProgram(self.id);
	}
    }
}

///
/// A program builder
///
pub struct ProgramBuilder {
    ///
    /// The program under construction
    ///
    program_id: GLuint,
    ///
    /// Attached shaders
    ///
    shaders: Vec<AttachedShader>,
}

impl ProgramBuilder {
    ///
    /// Creates a new program
    ///
    pub fn new() -> Result<ProgramBuilder, Error> {
	let id = unsafe {
	    gl::CreateProgram()
	};
	if id == 0 {
	    Err(Error::CreateProgram)
	} else {
	    Ok(ProgramBuilder {
		program_id: id,
		shaders: Vec::new(),
	    })
	}
    }

    ///
    /// Takes ownership of the shader and attaches it to the program
    ///
    pub fn attach(&mut self, shader: Shader) {
	self.shaders.push(shader.attach(self.program_id));
    }

    ///
    /// Links the program and releases ownership of attached shaders
    ///
    pub fn link(mut self) -> (Program, Vec<Shader>) {
	unsafe {
	    gl::LinkProgram(self.program_id);
	}
	let shaders = self.shaders.drain(..).map(AttachedShader::detach).collect();
	let program = Program {
	    id: self.program_id,
	};
	(program, shaders)
    }
}

impl Drop for ProgramBuilder {
    ///
    /// Drops the OpenGL resources associated with the program
    /// Attached shaders will be dropped automatically
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteProgram(self.program_id);
	}
    }
}

///
/// Errors that can occur when a program is created
///
pub enum Error {
    ///
    /// An openGL error happened when creating the program
    ///
    CreateProgram,
}
