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
use crate::graphics::color::Color;
use crate::graphics::font::Font;
use crate::graphics::program::{Program, Uniform4f32, UniformMatrix4f32, UniformInteger};
use crate::graphics::texture::Texture;
use crate::graphics::transform::Transform;
use crate::resource::Resources;

use std::path::PathBuf;

use sdl2::VideoSubsystem;
use sdl2::video::{GLContext, Window, WindowBuildError};

///
/// Program ID type
///
pub type ProgramId = usize;

///
/// Texture ID type
///
pub type TextureId = usize;

///
/// Vertex buffer ID type
///
pub type VertexBufferId = usize;

///
/// The graphics subsystem
///
pub struct Graphics {
    _window: Window,
    _gl_context: GLContext,
    programs: Resources<Program>,
    buffers: Resources<IndexedTriangles>,
    _fonts: Resources<Font>,
    textures: Resources<Texture>,
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
	    programs: programs,
	    buffers: buffers,
	    textures: textures,
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

    ///
    ///
    ///
    pub fn vertex_buffer_id(&self, name: &str) -> Result<VertexBufferId, Error> {
	self.buffers.id_by_name(name).ok_or(Error::NoVertexBuffer)
    }

    ///
    /// Returns the program ID for a specified name
    ///
    pub fn program_id(&self, name: &str) -> Result<ProgramId, Error> {
	self.programs.id_by_name(name).ok_or(Error::NoProgram)
    }

    ///
    /// Uses the program
    ///
    pub fn use_program(&self, program_id: ProgramId) -> Result<(), Error> {
	self.programs.get(program_id).ok_or(Error::NoProgram)?.use_program();	
	Ok(())
    }

    ///
    /// Creates a 4 x f32 tuple uniform 
    ///
    pub fn uniform_4f32(&self, program_id: ProgramId, name: &str) -> Result<Uniform4f32, Error> {
	Ok(self.programs.get(program_id).ok_or(Error::NoProgram)?.uniform_4f32(name)?)
    }

    ///
    /// Creates a 4 x 4 f32 matrix uniform 
    ///
    pub fn uniform_matrix_4f32(&self, program_id: ProgramId, name: &str) -> Result<UniformMatrix4f32, Error> {
	Ok(self.programs.get(program_id).ok_or(Error::NoProgram)?.uniform_matrix_4f32(name)?)
    }

    ///
    /// Creates an integer uniform
    ///
    pub fn uniform_integer(&self, program_id: ProgramId, name: &str) -> Result<UniformInteger, Error> {
	Ok(self.programs.get(program_id).ok_or(Error::NoProgram)?.uniform_integer(name)?)
    }

    ///
    /// Binds a texture
    ///
    pub fn bind_texture(&self, texture_id: TextureId) -> Result<(), Error> {
	self.textures.get(texture_id).ok_or(Error::NoTexture)?.bind();
	Ok(())
    }
    
    ///
    /// Draws a vertex buffer
    ///
    pub fn draw_vertex_buffer(&self, vertex_buffer_id: VertexBufferId) -> Result<(), Error> {
	self.buffers.get(vertex_buffer_id).ok_or(Error::NoVertexBuffer)?.draw();
	Ok(())
    }
}

///
/// Errors that occur when using the graphics subsystem
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

    ///
    /// No program found for the specified ID
    ///
    NoProgram,

    ///
    /// No texture found for the specified ID
    ///
    NoTexture,
    
    ///
    /// No vertex buffer found for the specified ID
    ///
    NoVertexBuffer,
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
