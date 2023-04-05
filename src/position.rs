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
/// A 2D position
///
#[derive(Clone)]
pub struct Position {
    ///
    /// The x coordinate
    ///
    pub x: f32,

    ///
    /// The y coordinate
    ///
    pub y: f32,
}

impl Position {
    ///
    /// Creates a new position
    ///
    pub fn new(x: f32, y: f32) -> Position {
	Position {
	    x,
	    y,
	}
    }
}

impl Default for Position {
    ///
    /// Creates a new position at the origin
    ///
    fn default() -> Position {
	Position {
	    x: 0.0,
	    y: 0.0,
	}
    }
}
