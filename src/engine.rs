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
use crate::graphics::{Error as GraphicsError, Graphics};

use sdl2::{Sdl, VideoSubsystem};

///
/// The engine subsystem
/// Owns the handles representing the immutable data and subsystem
///
pub struct Engine {
    ///
    /// SDL context handle
    ///
    _sdl: Sdl,
    ///
    /// SDL video system handle
    ///
    video: VideoSubsystem,
}

impl Engine {
    ///
    /// Creates a new instance of the engine
    ///
    pub fn new() -> Result<Engine, Error> {
	let sdl = sdl2::init().map_err(|msg| Error::InitContext(msg))?;
	let video = sdl.video().map_err(|msg| Error::InitVideo(msg))?;
	Ok(Engine {
	    _sdl: sdl,
	    video,
	})
    }

    ///
    /// Creates the graphics subsystem
    ///
    pub fn create_graphics(&self, settings: &Settings) -> Result<Graphics, GraphicsError> {
	Graphics::new(&self.video, settings)
    }

}

///
/// All errors that can occur when using the engine subsystem
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// An error happened inside the SDL library when initializing the SDL context
    ///
    InitContext(String),
    ///
    /// An error happened inside the SDL library when initializing the video subsystem
    ///
    InitVideo(String),
}
