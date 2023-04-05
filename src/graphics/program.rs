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

use crate::graphics::shader::{Shader, ShaderKind};
use crate::resource::Resources;

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use gl::types::GLuint;
use serde::Deserialize;

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

    ///
    /// Loads a set of programs
    ///
    pub fn load_from_config(path: &mut PathBuf) -> Result<Resources<Program>, Error> {
	let mut config: ProgramAndShaderConfiguration = crate::configuration::load(path)?;
	path.pop();
	let mut shaders = HashMap::new();
	for (name, shader) in config.shaders.drain() {
	    path.push(&name);
	    shaders.insert(name, Rc::from(Shader::load(path, shader.kind)?));
	    path.pop();
	}
	let mut programs = Resources::new();
	for (name, program) in config.programs.drain() {
	    let mut builder = ProgramBuilder::new()?;
	    for shader_name in program.shaders.iter() {
		builder.attach(shaders.get(shader_name).ok_or(Error::NoShader((*shader_name).clone()))?.clone());
	    }
	    programs.insert(name, builder.link())?;
	}
	Ok(programs)
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
struct ProgramBuilder {
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
    ///
    /// No shader found for the specified name
    ///
    NoShader(String),
    ///
    /// A resource error occurred
    ///
    Resource(crate::resource::Error),
    ///
    /// A configuration error occurred
    ///
    Configuration(crate::configuration::Error),
    ///
    /// A shader error occurred
    ///
    Shader(crate::graphics::shader::Error),
}

impl From<crate::resource::Error> for Error {
    ///
    /// Converts a resource error into a program error
    ///
    fn from(e: crate::resource::Error) -> Error {
	Error::Resource(e)
    }
}

impl From<crate::configuration::Error> for Error {
    ///
    /// Converts a configuration error into a program error
    ///
    fn from(e: crate::configuration::Error) -> Error {
	Error::Configuration(e)
    }
}

impl From<crate::graphics::shader::Error> for Error {
    ///
    /// Converts a shader error into a program error
    ///
    fn from(e: crate::graphics::shader::Error) -> Error {
	Error::Shader(e)
    }
}

///
/// Program and shader configuration
///
#[derive(Deserialize)]
struct ProgramAndShaderConfiguration {
    ///
    /// Shaders
    ///
    shaders: HashMap<String, ShaderConfiguration>,
    ///
    /// Programs
    ///
    programs: HashMap<String, ProgramConfiguration>,
}

///
/// Models a single program's configuration
///
#[derive(Deserialize)]
struct ProgramConfiguration {
    ///
    /// The names of the attached shaders
    ///
    shaders: Vec<String>,
}

///
/// A shader's configuration
///
#[derive(Deserialize)]
struct ShaderConfiguration {
    ///
    /// The kind of shader
    ///
    kind: ShaderKind,
}