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

use crate::dimension::Dimension;
use crate::graphics::texture::Texture;
use crate::resource::Resources;
use crate::vector::Vector;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

///
/// A font
///
pub struct Font {
    ///
    /// The regular face
    ///
    regular: Face,

    ///
    /// The italic face, if any
    ///
    italic: Option<Face>,

    ///
    /// The bold face, if any
    ///
    bold: Option<Face>,

    ///
    /// The bold italic face, if any
    ///
    bold_italic: Option<Face>,
}

impl Font {
    ///
    /// Loads a list of fonts from a folder
    ///
    pub fn load_from_folder(path: &mut PathBuf) -> Result<Resources<Font>, Error> {
	path.push("fonts.yaml");
	let mut config: HashMap<String, FontConfiguration> = crate::configuration::load(path)?;
	path.pop();
	
	let mut library = freetype::Library::init()?;
	let mut resources = Resources::new();
	for (name, font_config) in config.drain() {
	    resources.insert_from(name, || Font::load(&mut library, path, &font_config));
	}
	Ok(resources)
    }

    ///
    /// Loads a font
    ///
    fn load(library: &mut freetype::Library, path: &mut PathBuf, config: &FontConfiguration) -> Result<Font, Error> {
	let regular = Font::load_face(library, path, &config.regular, config.height)?;
	let bold = config.bold.as_ref().map(|file| Font::load_face(library, path, &file, config.height)).transpose()?;
	let italic = config.italic.as_ref().map(|file| Font::load_face(library, path, &file, config.height)).transpose()?;
	let bold_italic = config.bold_italic.as_ref().map(|file| Font::load_face(library, path, &file, config.height)).transpose()?;
	
	Ok(Font {
	    regular,
	    bold,
	    italic,
	    bold_italic,
	})
    }

    ///
    /// Loads a face from a specified file and sets the height in pixels of the textures as required
    ///
    fn load_face(library: &mut freetype::Library, path: &mut PathBuf, file: &String, height: u32) -> Result<Face, Error>{
	path.push(file);
	let face = Face::load(library, path, height)?;
	path.pop();
	Ok(face)
    }
}

///
/// A face
///
struct Face {
    ///
    /// The glyphs
    ///
    glyphs: Vec<Glyph>,
}

impl Face {
    ///
    /// Loads a face
    ///
    fn load(library: &mut freetype::Library, path: &PathBuf, height: u32) -> Result<Face, Error> {
	let mut face = library.new_face(path, 0)?;

	// setting width to zero causes it to be computed from the height
	face.set_pixel_sizes(0, height)?;

	let mut glyphs = Vec::with_capacity(0x100);

	// Load all code points in the first two blocks (this should be enough)
	for code_point in 0..0x100 {
	    glyphs.push(Glyph::load(&mut face, code_point)?);
	}
	Ok(Face {
	    glyphs,
	})
    }
}

///
/// A glyph
///
struct Glyph {
    ///
    /// The texture
    ///
    texture: Texture,

    ///
    /// The bearing (
    ///
    bearing: Vector,

    ///
    /// The size
    ///
    size: Dimension,

    ///
    /// The horizontal advance
    ///
    advance: f32,
}

impl Glyph {
    ///
    /// Loads a glyph
    ///
    fn load(face: &mut freetype::Face, code_point: usize) -> Result<Glyph, Error> {
	face.load_char(code_point, freetype::face::LoadFlag::RENDER)?;
	let texture_id = 0;
	let metrics = face.glyph().metrics();
	let bitmap = face.glyph().bitmap();
	Ok(Glyph {
	    texture: Texture::from_buffer(bitmap.buffer(), bitmap.width(), bitmap.rows())?,
	    bearing: Vector::new(metrics.horiBearingX as f32, metrics.vertBearingY as f32),
	    size: Dimension::new(metrics.width as f32, metrics.height as f32),
	    advance: metrics.horiAdvance as f32,
	})
    }
}

///
/// Errors that can occur when using fonts
///
#[derive(Debug)]
pub enum Error {
    ///
    /// Could not load configuration
    ///
    Configuration(crate::configuration::Error),

    ///
    /// A free type error
    ///
    Freetype(freetype::Error),

    ///
    /// A resource error occurred
    ///
    Resource(crate::resource::Error),
    
    ///
    /// Could not create the texture
    ///
    Texture(crate::graphics::texture::Error),
}

impl From<crate::configuration::Error> for Error {
    ///
    /// Converts a configuration error into a font error
    ///
    fn from(e: crate::configuration::Error) -> Error {
	Error::Configuration(e)
    }
}

impl From<freetype::Error> for Error {
    ///
    /// Converts a configuration error into a font error
    ///
    fn from(e: freetype::Error) -> Error {
	Error::Freetype(e)
    }
}

impl From<crate::resource::Error> for Error {
    ///
    /// Converts a resource error into a font error
    ///
    fn from(e: crate::resource::Error) -> Error {
	Error::Resource(e)
    }
}

impl From<crate::graphics::texture::Error> for Error {
    ///
    /// Converts a texture error into a font error
    ///
    fn from(e: crate::graphics::texture::Error) -> Error {
	Error::Texture(e)
    }
}

///
/// Font configuration model
///
#[derive(Deserialize)]
struct FontConfiguration {
    ///
    /// The height in pixels of the textures
    ///
    height: u32,

    ///
    /// The regular font file
    ///
    regular: String,

    ///
    /// The italic font file
    ///
    italic: Option<String>,

    ///
    /// The bold font file
    ///
    bold: Option<String>,

    ///
    /// The bold italic font file
    ///
    bold_italic: Option<String>,
}
