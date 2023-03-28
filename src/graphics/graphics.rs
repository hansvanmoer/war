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
use super::shader::ShaderKind;

use sdl2::VideoSubsystem;
use sdl2::video::{GLContext, Window, WindowBuildError};
use serde::Deserialize;

///
/// The graphics subsystem
///
pub struct Graphics {
    _window: Window,
    _gl_context: GLContext,
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
	Ok(Graphics {
	    _window: window,
	    _gl_context: gl_context,
	})
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

///
/// Models the graphics pipeline configuration file
///
#[derive(Deserialize)]
struct GraphicsConfiguration {
    ///
    /// The shaders
    ///
    shaders: Vec<ShaderConfiguration>,
    
    ///
    /// The programs
    ///
    programs: Vec<ProgramConfiguration>,
}

///
/// Models a single program's configuration
///
#[derive(Deserialize)]
struct ProgramConfiguration {
    ///
    /// The program's unique name
    ///
    name: String,

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
    /// The shader's unique name
    ///
    name: String,

    ///
    /// The kind of shader
    ///
    kind: ShaderKind,
}
