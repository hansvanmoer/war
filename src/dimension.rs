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
/// A 2D size tuple
///
#[derive(Clone)]
pub struct Dimension {
    ///
    /// The width, always positive
    ///
    width: f32,

    ///
    /// The height, always positive
    ///
    height: f32,
}

impl Dimension {
    ///
    /// Creates a new dimension
    ///
    pub fn new(width: f32, height: f32) -> Dimension {
	let width = if width < 0.0 {
	    - width
	} else {
	    width
	};
	let height = if height < 0.0 {
	    - height
	} else {
	    height
	};
	Dimension {
	    width,
	    height,
	}
    }

    ///
    /// Returns the width
    ///
    pub fn width(&self) -> f32 {
	self.width
    }

    ///
    /// Returns the height
    ///
    pub fn height(&self) -> f32 {
	self.height
    }
}

impl Default for Dimension {
    ///
    /// Creates a new zero sized dimension
    ///
    fn default() -> Dimension {
	Dimension {
	    width: 0.0,
	    height: 0.0,
	}
    }
}
