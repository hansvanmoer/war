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

use std::path::PathBuf;

use crate::graphics::Color;
use crate::validation::{Error as ValidationError, ValidateInto, Validator};

use serde::Deserialize;

///
/// A UI style
///
pub struct Style {
    ///
    /// The button style
    ///
    button: ButtonStyle,

    ///
    /// The container style
    ///
    container: ContainerStyle,
    
    ///
    /// Font name
    ///
    font_name: String,

    ///
    /// Font size
    ///
    font_size: f32,
}

impl Style {
    ///
    /// Loads a widget style
    ///
    pub fn load(path: &PathBuf) -> Result<Style, Error> {
	let model: StyleConfiguration = crate::configuration::load(path)?;
	let mut validator = Validator::new();
	Ok(validator.validate_into(model)?)
    }

    ///
    /// The button style 
    ///
    pub fn button(&self) -> &ButtonStyle {
	&self.button
    }

    ///
    /// The container style
    ///
    pub fn container(&self) -> &ContainerStyle {
	&self.container
    }
    
    ///
    /// The name of the font to be used
    ///
    pub fn font_name(&self) -> &str {
	&self.font_name
    }

    ///
    /// The size of the font
    ///
    pub fn font_size(&self) -> f32 {
	self.font_size
    }
}

///
/// Errors that can occur when loading the style
///
pub enum Error {
    ///
    /// A configuration error occurred
    ///
    Configuration(crate::configuration::Error),

    ///
    /// A validation error occurred
    ///
    Validation(ValidationError),
}

impl From<crate::configuration::Error> for Error {
    ///
    /// Converts a configuration error into a style error
    ///
    fn from(e: crate::configuration::Error) -> Error {
	Error::Configuration(e)
    }
}

impl From<ValidationError> for Error {
    ///
    /// Converts a validation error into a style error
    ///
    fn from(e: ValidationError) -> Error {
	Error::Validation(e)
    }
}

///
/// A button style
///
pub struct ButtonStyle {
    ///
    /// The background color
    ///
    background: Color,

    ///
    /// Foreground color
    ///
    foreground: Color,

    ///
    /// Inner margins
    ///
    margins: Margins,
}

impl ButtonStyle {
    ///
    /// The background color
    ///
    pub fn background(&self) -> &Color {
	&self.background
    }

    ///
    /// The foreground color
    ///
    pub fn foreground(&self) -> &Color {
	&self.foreground
    }

    ///
    /// The margins around the text or icon
    ///
    pub fn margins(&self) -> &Margins {
	&self.margins
    }
}

///
/// Style for container types
///
pub struct ContainerStyle {
    ///
    /// The outer margins of child elements
    ///
    margins: Margins,
}

impl ContainerStyle {
    ///
    /// Returns the outer margins for child elements
    ///
    pub fn margins(&self) -> &Margins {
	&self.margins
    }
}

///
/// Margins of a widget
///
#[derive(Clone)]
pub struct Margins {
    ///
    /// The left margin
    ///
    left: f32,

    ///
    /// The right margin
    ///
    right: f32,

    ///
    /// The top margin
    ///
    top: f32,

    ///
    /// The bottom margin
    ///
    bottom: f32,
}

impl Margins {
    ///
    /// Creates a new margins object
    ///
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Margins {
	Margins {
	    left: if left < 0.0 {
		- left
	    } else {
		left
	    },
	    right: if right < 0.0 {
		- right
	    } else {
		right
	    },
	    bottom: if bottom < 0.0 {
		- bottom
	    } else {
		bottom
	    },
	    top: if top < 0.0 {
		- top
	    } else {
		top
	    }
	}
    }

    ///
    /// Returns the left margin
    ///
    pub fn left(&self) -> f32 {
	self.left
    }

    ///
    /// Returns the right margin
    ///
    pub fn right(&self) -> f32 {
	self.right
    }

    ///
    /// Returns the top margin
    ///
    pub fn top(&self) -> f32 {
	self.top
    }

    ///
    /// Returns the bottom margin
    ///
    pub fn bottom(&self) -> f32 {
	self.bottom
    }

    ///
    /// Returns the total horizontal margin
    ///
    pub fn horizontal(&self) -> f32 {
	self.left + self.right
    }

    ///
    /// Returns the total vertical margin
    ///
    pub fn vertical(&self) -> f32 {
	self.top + self.bottom
    }
}

impl Default for Margins {
    ///
    /// Creates a zero margins object
    ///
    fn default() -> Margins {
	Margins {
	    left: 0.0,
	    right: 0.0,
	    bottom: 0.0,
	    top: 0.0,
	}
    }
}

///
/// A UI style configuration model
///
#[derive(Deserialize)]
pub struct StyleConfiguration {
    ///
    /// The button style
    ///
    button: ButtonStyleConfiguration,

    ///
    /// The container style
    ///
    container: ContainerStyleConfiguration,
    
    ///
    /// Font name
    ///
    font_name: String,

    ///
    /// Font size
    ///
    font_size: f32,
}

impl ValidateInto<Style> for StyleConfiguration {
    
    fn validate_into(self, v: &mut Validator) -> Result<Style, ValidationError> {
	Ok(Style {
	    button: v.validate_field_into("button", self.button)?,
	    container: v.validate_field_into("container", self.container)?,
	    font_name: v.validate_field("font_name", "must not be empty", self.font_name, |v| !v.is_empty())?,
	    font_size: v.validate_field("font size", "must be > 0", self.font_size, |v| *v > 0.0)?,
	})
    }
}

///
/// Button style configuration
///
#[derive(Deserialize)]
pub struct ButtonStyleConfiguration {
    ///
    /// The background color
    ///
    background: ColorConfiguration,

    ///
    /// Foreground color
    ///
    foreground: ColorConfiguration,

    ///
    /// Margins
    ///
    margins: MarginsConfiguration,
}

impl ValidateInto<ButtonStyle> for ButtonStyleConfiguration {

    fn validate_into(self, v: &mut Validator) -> Result<ButtonStyle, ValidationError> {
	Ok(ButtonStyle {
	    background: v.validate_field_into("background", self.background)?,
	    foreground: v.validate_field_into("foreground", self.foreground)?,
	    margins: v.validate_field_into("margins", self.margins)?,
	})
    }
}

///
/// Style configuration for container types
///
#[derive(Deserialize)]
pub struct ContainerStyleConfiguration {
    ///
    /// The outer margins of child elements
    ///
    margins: MarginsConfiguration,
}

impl ValidateInto<ContainerStyle> for ContainerStyleConfiguration {

    fn validate_into(self, v: &mut Validator) -> Result<ContainerStyle, ValidationError> {
	Ok(ContainerStyle {
	    margins: v.validate_field_into("margins", self.margins)?,
	})
    }
}

///
/// Color configuration model
///
#[derive(Deserialize)]
struct ColorConfiguration {
    ///
    /// The red channel
    ///
    red: f32,

    ///
    /// The green channel
    ///
    green: f32,

    ///
    /// The blue channel
    ///
    blue: f32,

    ///
    /// The alpha channel
    ///
    alpha: f32,
}

impl ValidateInto<Color> for ColorConfiguration {

    fn validate_into(self, v: &mut Validator) -> Result<Color, ValidationError> {
	Ok(Color::new(
	    v.validate_field("left", "must be between 0 and 1", self.red, |v| *v >= 0.0 && *v <= 1.0)?,
	    v.validate_field("green", "must be between 0 and 1", self.green, |v| *v >= 0.0 && *v <= 1.0)?,
	    v.validate_field("blue", "must be between 0 and 1", self.blue, |v| *v >= 0.0 && *v <= 1.0)?,
	    v.validate_field("alpha", "must be between 0 and 1", self.alpha, |v| *v >= 0.0 && *v <= 1.0)?
	))
    }
}

///
/// Margins configuration model
///
#[derive(Deserialize)]
struct MarginsConfiguration {
    ///
    /// The left margin
    ///
    left: f32,

    ///
    /// The right margin
    ///
    right: f32,

    ///
    /// The top margin
    ///
    top: f32,

    ///
    /// The bottom margin
    ///
    bottom: f32,
}

impl ValidateInto<Margins> for MarginsConfiguration {

    fn validate_into(self, v: &mut Validator) -> Result<Margins, ValidationError> {
	Ok(Margins {
	    left: v.validate_field("left", "must be >= 0", self.left, |v| *v >= 0.0)?,
	    right: v.validate_field("right", "must be >= 0", self.right, |v| *v >= 0.0)?,
	    top: v.validate_field("top", "must be >= 0", self.top, |v| *v >= 0.0)?,
	    bottom: v.validate_field("bottom", "must be >= 0", self.bottom, |v| *v >= 0.0)?,
	})
    }
 
}
