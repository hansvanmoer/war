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

use crate::resource::Resources;

use gl::types::GLuint;
use image::ImageError;

use std::collections::HashMap;
use std::path::PathBuf;

///
/// A texture
///
pub struct Texture {
    ///
    /// The internal OpenGL handle for this texture
    ///
    id: GLuint,
}

impl Texture {
    ///
    /// Loads a set of textures from a folder
    ///
    pub fn load_from_folder(path: &mut PathBuf) -> Result<Resources<Texture>, Error> {
	path.push("textures.yaml");
	let mut config: HashMap<String, String> = crate::configuration::load(path)?;
	path.pop();
	let mut resources = Resources::new();
	for (name, file) in config.drain() {
	    path.push(file);
	    let texture = Texture::load(path)?;
	    path.pop();
	    resources.insert(name, texture)?;
	}
	Ok(resources)
    }
    
    ///
    /// Creates a texture from a buffer
    ///
    pub fn from_buffer(buffer: &[u8], width: i32, height: i32) -> Result<Texture, Error> {
	if width < 0 {
	    Err(Error::BadWidth(width))
	} else if height < 0 {
	    Err(Error::BadHeight(height))
	} else {
	    
	    let mut id: GLuint = 0;
	    unsafe {
		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl::TEXTURE_2D, id);
		gl::TexImage2D(
		    gl::TEXTURE_2D,
		    0,
		    gl::RGBA as i32,
		    width,
		    height,
		    0,
		    gl::RGBA,
		    gl::UNSIGNED_BYTE,
		    buffer.as_ptr() as * const gl::types::GLvoid
		);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
	    }
	    
	    Ok(Texture {
		id,
	    })
	}
    }

    ///
    /// Loads a texture from a path
    ///
    pub fn load(path: &PathBuf) -> Result<Texture, Error> {
	let image = image::open(path)?.into_rgba8();

	Texture::from_buffer(image.as_raw().as_slice(), image.width() as i32, image.height() as i32)
    }
}

impl Drop for Texture {
    ///
    /// Drops the texture's OpenGL managed resources
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteTextures(1, &self.id);
	}
    }
}

///
/// Errors that can occur working with textures
///
#[derive(Debug)]
pub enum Error {
    ///
    /// An error occurred while loading the image
    ///
    Image(ImageError),

    ///
    /// Invalid buffer width
    ///
    BadWidth(i32),

    ///
    /// Invalid buffer height
    ///
    BadHeight(i32),

    ///
    /// Configuration error
    ///
    Configuration(crate::configuration::Error),

    ///
    /// Resource error
    ///
    Resource(crate::resource::Error),
}

impl From<ImageError> for Error {
    ///
    /// Converts an image error into a texture error
    ///
    fn from(e: ImageError) -> Error {
	Error::Image(e)
    }
}

impl From<crate::configuration::Error> for Error {
    ///
    /// Converts a configuration error into a texture error
    ///
    fn from(e: crate::configuration::Error) -> Error {
	Error::Configuration(e)
    }
}

impl From<crate::resource::Error> for Error {
    ///
    /// Converts a resource error into a texture error
    ///
    fn from(e: crate::resource::Error) -> Error {
	Error::Resource(e)
    }
}
