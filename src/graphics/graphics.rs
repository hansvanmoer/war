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
use crate::graphics::font::Font;
use crate::graphics::program::Program;
use crate::graphics::texture::Texture;
use crate::resource::Resources;

use std::path::PathBuf;

use sdl2::VideoSubsystem;
use sdl2::video::{GLContext, Window, WindowBuildError};

///
/// The graphics subsystem
///
pub struct Graphics {
    _window: Window,
    _gl_context: GLContext,
    _programs: Resources<Program>,
    _buffers: Resources<IndexedTriangles>,
    _fonts: Resources<Font>,
    _textures: Resources<Texture>,
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
	let programs = Graphics::load_programs(&mut path)?;
	let buffers = Graphics::load_buffers(&mut path)?;
	let textures = Graphics::load_textures(&mut path)?;
	let fonts = Graphics::load_fonts(&mut path)?;
	
	Ok(Graphics {
	    _window: window,
	    _gl_context: gl_context,
	    _programs: programs,
	    _buffers: buffers,
	    _textures: textures,
	    _fonts: fonts,
	})
    }

    ///
    /// Loads the graphics pipelines
    ///
    fn load_programs(path: &mut PathBuf) -> Result<Resources<Program>, Error> {
	path.push("shaders");
	let programs = Program::load_from_folder(path)?;
	path.pop();
	Ok(programs)
    }

    ///
    /// Loads the vertex buffers
    ///
    fn load_buffers(path: &mut PathBuf) -> Result<Resources<IndexedTriangles>, Error> {
	path.push("buffers");
	let buffers = IndexedTriangles::load_from_folder(path)?;
	path.pop();
	Ok(buffers)
    }

    ///
    /// Loads the textures
    ///
    fn load_textures(path: &mut PathBuf) -> Result<Resources<Texture>, Error> {
	path.push("textures");
	let textures = Texture::load_from_folder(path)?;
	path.pop();
	Ok(textures)
    }
    
    ///
    /// Loads the fonts
    ///
    fn load_fonts(path: &mut PathBuf) -> Result<Resources<Font>, Error> {
	path.push("fonts");
	let fonts = Font::load_from_folder(path)?;
	path.pop();
	Ok(fonts)
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

    ///
    /// Texture error
    ///
    Texture(crate::graphics::texture::Error),

    ///
    /// Font error
    ///
    Font(crate::graphics::font::Error),
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

impl From<crate::graphics::texture::Error> for Error {
    ///
    /// Converts a texture error into a graphics error
    ///
    fn from(e: crate::graphics::texture::Error) -> Error {
	Error::Texture(e)
    }
}

impl From<crate::graphics::font::Error> for Error {
    ///
    /// Converts a font error into a graphics error
    ///
    fn from(e: crate::graphics::font::Error) -> Error {
	Error::Font(e)
    }
}
