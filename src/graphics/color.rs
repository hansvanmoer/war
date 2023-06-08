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

use crate::graphics::program::Uniform4f32;

///
/// A color tuple
///
#[derive(Debug, PartialEq)]
pub struct Color {
    ///
    /// The normalized red channel
    ///
    red: f32,

    ///
    /// The normalized green channel
    ///
    green: f32,

    ///
    /// The normalized blue channel
    ///
    blue: f32,

    ///
    /// The normalized alpha channel
    ///
    alpha: f32,
}

impl Color {
    ///
    /// Creates a new color
    ///
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
	Color {
	    red: if red > 1.0 {
		1.0
	    } else if red < 0.0 {
		0.0
	    } else {
		red
	    },
	    green: if green > 1.0 {
		1.0
	    } else if green < 0.0 {
		0.0
	    } else {
		green
	    },
	    blue: if blue > 1.0 {
		1.0
	    } else if blue < 0.0 {
		0.0
	    } else {
		blue
	    },
	    alpha: if alpha > 1.0 {
		1.0
	    } else if alpha < 0.0 {
		0.0
	    } else {
		alpha
	    }
	}
    }

    ///
    /// Returns the red channel
    ///
    pub fn red(&self) -> f32 {
	self.red
    }

    ///
    /// Returns the green channel
    ///
    pub fn green(&self) -> f32 {
	self.green
    }

    ///
    /// Returns the blue channel
    ///
    pub fn blue(&self) -> f32 {
	self.blue
    }

    ///
    /// Returns the alpha channel
    ///
    pub fn alpha(&self) -> f32 {
	self.alpha
    }

    ///
    /// Copies this type into a uniform variable
    ///
    pub fn copy_to_uniform(&self, uniform: &mut Uniform4f32) {
	uniform.set(self.red(), self.green(), self.blue(), self.alpha());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_new() {
	assert_eq!(Color {
	    red: 1.0,
	    green: 1.0,
	    blue: 1.0,
	    alpha: 1.0,
	}, Color::new(2.0, 1.1, 3.0, 4.5));

	assert_eq!(Color {
	    red: 0.0,
	    green: 0.0,
	    blue: 0.0,
	    alpha: 0.0,
	}, Color::new(-2.0, -1.1, -0.3, -4.5));

	assert_eq!(Color {
	    red: 0.8,
	    green: 1.0,
	    blue: 0.4,
	    alpha: 0.2,
	}, Color::new(0.8, 1.0, 0.4, 0.2));
	
    }
}
