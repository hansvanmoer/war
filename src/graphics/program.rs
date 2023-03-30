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

use super::shader::Shader;

use std::rc::Rc;

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

impl Program {
    ///
    /// Uses a program
    ///
    pub fn run(&self) {
	unsafe {
	    gl::UseProgram(self.id);
	}
    }
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
    shaders: Vec<Rc<Shader>>,
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
    pub fn attach(&mut self, shader: Rc<Shader>) {
	shader.attach(self.program_id);
	self.shaders.push(shader);
    }

    ///
    /// Links the program and releases ownership of attached shaders
    ///
    pub fn link(mut self) -> Program {
	unsafe {
	    gl::LinkProgram(self.program_id);
	}
	self.shaders.drain(..).for_each(|s| s.detach(self.program_id));
	Program {
	    id: self.program_id,
	}
    }
}

impl Drop for ProgramBuilder {
    ///
    /// Drops the OpenGL resources associated with the program
    /// Attached shaders will be dropped automatically
    ///
    fn drop(&mut self) {
	self.shaders.drain(..).for_each(|s| s.detach(self.program_id));
	unsafe {
	    gl::DeleteProgram(self.program_id);
	}
    }
}

///
/// Errors that can occur when a program is created
///
#[derive(Debug)]
pub enum Error {
    ///
    /// An openGL error happened when creating the program
    ///
    CreateProgram,
}
