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

use crate::settings::Settings;
use crate::graphics::buffer::IndexedTriangles;
use crate::graphics::program::{Program, ProgramBuilder};
use crate::graphics::shader::{Shader, ShaderKind};

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use sdl2::VideoSubsystem;
use sdl2::video::{GLContext, Window, WindowBuildError};
use serde::Deserialize;

///
/// The graphics subsystem
///
pub struct Graphics {
    _window: Window,
    _gl_context: GLContext,
    _programs: Vec<Program>,
    _programs_by_name: HashMap<String, usize>,
    _buffers: Vec<IndexedTriangles>,
    _buffers_by_name: HashMap<String, usize>,
}

impl Graphics {
    ///
    /// Initializes the graphics subsystem
    ///
    pub fn new(video: &VideoSubsystem, settings: &Settings) -> Result<Graphics, Error> {
	let window = video.window("The Hundred Years War", settings.window_width(), settings.window_height())
	    .build()?;
	let gl_context = window.gl_create_context().map_err(|msg| Error::Sdl(msg))?;
	gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

	let mut path = settings.create_data_path();
	path.push("graphics.yaml");
	let config: GraphicsConfiguration = crate::configuration::load(&path)?;
	path.pop();
	
	let mut programs = Vec::new();
	let mut programs_by_name = HashMap::new();
	for (name, program) in Graphics::load_programs(&mut path, &config)?.drain() {
	    let id = programs.len();
	    programs.push(program);
	    programs_by_name.insert(name, id);
	}

	let mut buffers = Vec::new();
	let mut buffers_by_name = HashMap::new();
	for (name, buffer) in Graphics::load_buffers(&mut path, &config)?.drain() {
	    let id = buffers.len();
	    buffers.push(buffer);
	    buffers_by_name.insert(name, id);
	}
	
	Ok(Graphics {
	    _window: window,
	    _gl_context: gl_context,
	    _programs: programs,
	    _programs_by_name: programs_by_name,
	    _buffers: buffers,
	    _buffers_by_name: buffers_by_name,
	})
    }

    ///
    /// Loads the graphics pipelines
    ///
    fn load_programs(path: &mut PathBuf, config: &GraphicsConfiguration) -> Result<HashMap<String, Program>, Error> {
	let shaders = Graphics::load_shaders(path, &config)?;
	let mut programs = HashMap::new();
	for (name, program_config) in config.programs.iter() {
	    let mut builder = ProgramBuilder::new()?;
	    for shader_name in program_config.shaders.iter() {
		builder.attach(shaders.get(shader_name).ok_or_else(|| Error::NoShader((*shader_name).clone()))?.clone());
	    }
	    programs.insert((*name).clone(), builder.link());
	}
	Ok(programs)
    }

    ///
    /// Loads the shaders
    ///
    fn load_shaders(path: &mut PathBuf, config: &GraphicsConfiguration) -> Result<HashMap<String, Rc<Shader>>, Error> {
	path.push("shaders");
	let mut result = HashMap::with_capacity(config.shaders.len());
	for (name, shader_config) in config.shaders.iter() {
	    path.push(name);
	    let shader = Rc::from(Shader::load(path, shader_config.kind.clone())?);
	    path.pop();
	    result.insert(name.clone(), shader);
	}
	path.pop();
	Ok(result)
    }

    ///
    /// Loads the vertex buffers
    ///
    fn load_buffers(path: &mut PathBuf, config: &GraphicsConfiguration) -> Result<HashMap<String, IndexedTriangles>, Error> {
	path.push("buffers");
	let mut result = HashMap::new();
	for name in config.buffers.iter() {
	    path.push(format!("{}.yaml", name));
	    result.insert((*name).clone(), IndexedTriangles::load(&path)?);
	    path.pop();
	}
	path.pop();
	Ok(result)
    }
}

///
///
///
#[derive(Debug)]
pub enum Error {
    ///
    /// The window width was invalid
    ///
    BadWindowWidth,
    ///
    /// The window height was invalid
    ///
    BadWindowHeight,
    ///
    /// The window title was invalid
    ///
    BadWindowTitle,
    /// 
    /// An SDL error occurred when the window was created
    ///
    Sdl(String),
    ///
    /// Configuration error
    ///
    Configuration(crate::configuration::Error),
    ///
    /// Shader not loaded
    ///
    NoShader(String),
    ///
    /// Shader error
    ///
    Shader(crate::graphics::shader::Error),
    ///
    /// Program error
    ///
    Program(crate::graphics::program::Error),
    ///
    /// Buffer error
    ///
    Buffer(crate::graphics::buffer::Error),
}

impl From<WindowBuildError> for Error {
    ///
    /// Converts a window build error to a form that can be formatted and compared
    ///
    fn from(e: WindowBuildError) -> Error {
	match e {
	    WindowBuildError::HeightOverflows(_) => Error::BadWindowHeight,
	    WindowBuildError::WidthOverflows(_) => Error::BadWindowWidth,
	    WindowBuildError::InvalidTitle(_) => Error::BadWindowTitle,
	    WindowBuildError::SdlError(msg) => Error::Sdl(msg),
	}
    }
}

impl From<crate::configuration::Error> for Error {
    ///
    /// Converts a configuration error into a graphics error
    ///
    fn from(e: crate::configuration::Error) -> Error {
	Error::Configuration(e)
    }
}

impl From<crate::graphics::shader::Error> for Error {
    ///
    /// Converts a shader error into a graphics error
    ///
    fn from(e: crate::graphics::shader::Error) -> Error {
	Error::Shader(e)
    }
}

impl From<crate::graphics::program::Error> for Error {
    ///
    /// Converts a program error into a graphics error
    ///
    fn from(e: crate::graphics::program::Error) -> Error {
	Error::Program(e)
    }
}

impl From<crate::graphics::buffer::Error> for Error {
    ///
    /// Converts a vertex buffer error into a graphics error
    ///
    fn from(e: crate::graphics::buffer::Error) -> Error {
	Error::Buffer(e)
    }
}

///
/// Models the graphics pipeline configuration file
///
#[derive(Deserialize)]
struct GraphicsConfiguration {
    ///
    /// The shaders
    ///
    shaders: HashMap<String, ShaderConfiguration>,
    
    ///
    /// The programs
    ///
    programs: HashMap<String, ProgramConfiguration>,

    ///
    /// The buffers
    ///
    buffers: Vec<String>,
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
