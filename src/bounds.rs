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
/// A 2D bounding box
///
pub struct Bounds {
    ///
    /// The lower bound on the x axis
    ///
    left: f32,

    ///
    /// The upper bound on the x axis
    ///
    right: f32,

    ///
    /// The lower bound on the y axis
    ///
    bottom: f32,

    ///
    /// The upper bound on the y axis
    ///
    top: f32,
}

impl Bounds {
    ///
    /// Creates a new 2D bounding box
    ///
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32) -> Bounds {
	let (left, right) = if x1 > x2 {
	    (x2, x1)
	} else {
	    (x1, x2)
	};
	let (bottom, top) = if y1 > y2 {
	    (y2, y1)
	} else {
	    (y1, y2)
	};
	Bounds {
	    left,
	    right,
	    bottom,
	    top,
	}
    }
}

impl Default for Bounds {
    ///
    /// Creates a new bounding box at the origin with a zero size
    ///
    fn default() -> Bounds {
	Bounds {
	    left: 0.0,
	    right: 0.0,
	    bottom: 0.0,
	    top: 0.0,
	}
    }
}
