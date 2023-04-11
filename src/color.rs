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

///
/// A color 
///
#[derive(Debug, PartialEq)]
pub struct Color {
    ///
    /// The normalized red channel value
    ///
    red: f32,
    
    ///
    /// The normalized green channel value
    ///
    green: f32,

    ///
    /// The normalized blue channel value
    ///
    blue: f32,

    ///
    /// The normalized alpha channel value
    ///
    alpha: f32,
}

impl Color {
    ///
    /// Creates a new color
    ///
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
	Color {
	    red: if red <= 1.0 {
		red
	    } else {
		1.0
	    },
	    green: if green <= 1.0 {
		green
	    } else {
		1.0
	    },
	    blue: if blue <= 1.0 {
		blue
	    } else {
		1.0
	    },
	    alpha: if alpha <= 1.0 {
		alpha
	    } else {
		1.0
	    },
	}
    }
}

impl Default for Color {
    ///
    /// Creates the default color (fully transparent black)
    ///
    fn default() -> Color {
	Color {
	    red: 0.0,
	    green: 0.0,
	    blue: 0.0,
	    alpha: 0.0,
	}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn color_default() {
	let expected = Color {
	    red: 0.0,
	    green: 0.0,
	    blue: 0.0,
	    alpha: 0.0,
	};
	assert_eq!(expected, Color::default());
    }

    #[test]
    fn color_new() {
	let expected = Color {
	    red: 0.5,
	    green: 0.6,
	    blue: 0.7,
	    alpha: 0.8,
	};
	assert_eq!(expected, Color::new(0.5, 0.6, 0.7, 0.8));
    }
    
    #[test]
    fn color_new_overflow() {
	let expected = Color {
	    red: 1.0,
	    green: 1.0,
	    blue: 1.0,
	    alpha: 1.0,
	};
	assert_eq!(expected, Color::new(1.5, 1.6, 1.7, 1.8));
    }
}
