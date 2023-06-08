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

use crate::graphics::{Color, Graphics, ProgramId, Transform, Uniform4f32, UniformMatrix4f32, VertexBufferId};

///
/// A renderer for UI elements
///
pub struct Renderer<'a> {
    ///
    /// The graphics subsystem
    ///
    graphics: &'a Graphics,

    ///
    /// The filled rectangle renderer
    ///
    filled_rectangle: FilledRectangleRenderer,
}

impl<'a> Renderer<'a> {
    ///
    /// Creates a new renderer for the UI
    ///
    pub fn new(graphics: &'a Graphics) -> Result<Renderer<'a>, Error> {
	Ok(Renderer {
	    graphics,
	    filled_rectangle: FilledRectangleRenderer::new(graphics)?,
	})
    }

    ///
    /// Fill a rectangle with the specified color
    ///
    pub fn fill_rectangle(&mut self, left: f32, right: f32, top: f32, bottom: f32, color: &Color) {
	self.filled_rectangle.render(&self.graphics, left, right, top, bottom, color);
    }
}

///
/// A renderer for filled rectangles
///
struct FilledRectangleRenderer {
    ///
    /// The program ID
    ///
    program: ProgramId,

    ///
    /// The vertex buffer ID
    ///
    vertex_buffer_id: VertexBufferId,
    
    ///
    /// The fill color uniform
    ///
    fill_color: Uniform4f32,

    ///
    /// The transform uniform
    ///
    transform: UniformMatrix4f32,
}

impl FilledRectangleRenderer {
    ///
    /// Creates the filled rectangle renderer
    ///
    fn new(graphics: &Graphics) -> Result<FilledRectangleRenderer, Error> {
	let program = graphics.program_id("ui_filled_rectangle")?;
	Ok(FilledRectangleRenderer {
	    program,
	    vertex_buffer_id: graphics.vertex_buffer_id("rectangle")?,
	    fill_color: graphics.uniform_4f32(program, "fill_color")?,
	    transform: graphics.uniform_matrix_4f32(program, "transform")?,
	})
    }

    ///
    /// Fill a rectangle with the specified color
    ///
    pub fn render(&mut self, graphics: &Graphics, left: f32, right: f32, top: f32, bottom: f32, color: &Color) {
	let width = right - left;
	let height = bottom - top;
	let transform = Transform::scale(1.0 / width, 1.0 + height, 1.0) * Transform::translate(left, right, 0.0);
	transform.copy_to_uniform(&mut self.transform);
	color.copy_to_uniform(&mut self.fill_color);
	graphics.draw_vertex_buffer(self.vertex_buffer_id);
    }
}

///
/// Drawing error
///
pub enum Error {
    ///
    /// A graphics error occurred
    ///
    Graphics(crate::graphics::Error),
}

impl From<crate::graphics::Error> for Error {
    ///
    ///  Converts a graphics error into a UI drawing error
    ///
    fn from(e: crate::graphics::Error) -> Error {
	Error::Graphics(e)
    }
}
