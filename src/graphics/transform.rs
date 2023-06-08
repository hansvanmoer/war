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

use crate::graphics::program::UniformMatrix4f32;

use std::ops::Mul;

///
/// A 3D transformation
///
pub struct Transform {
    ///
    /// The transformation matrix
    ///
    matrix: [f32; 16],
}

impl Transform {

    ///
    /// The identity transform
    ///
    pub fn identity() -> Transform {
	Transform {
	    matrix: [
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	    ],
	}
    }

    ///
    /// A scale transform
    ///
    pub fn scale(x: f32, y: f32, z: f32) -> Transform {
	Transform {
	    matrix: [
		x, 0.0, 0.0, 0.0,
		0.0, y, 0.0, 0.0,
		0.0, 0.0, z, 0.0,
		0.0, 0.0, 0.0, 1.0,
	    ],
	}
    }

    ///
    /// A translation transform
    ///
    pub fn translate(x: f32, y: f32, z: f32) -> Transform {
	Transform {
	    matrix: [
		1.0, 0.0, 0.0, x,
		0.0, 1.0, 0.0, y,
		0.0, 0.0, 1.0, z,
		0.0, 0.0, 0.0, 1.0,
	    ],
	}
    }
    
    ///
    /// Copies the transform to the uniform variable
    ///
    pub fn copyToUniform(&self, uniform: &mut UniformMatrix4f32) {
	uniform.set(&self.matrix);
    }
    
}

impl Mul for Transform {

    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
	let mut matrix = [0.0; 16];
	for r in 0..4 {
	    for c in 0..4 {
		let mut value = 0.0;
		for i in 0..4 {
		    value += other.matrix[r * 4 + i] * self.matrix[i * 4 + c];
		}
		matrix[r * 4 + c] = value;
	    }
	}
	Transform {
	    matrix,
	}
    }
} 
