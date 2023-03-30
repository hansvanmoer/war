use gl::types::GLuint;

///
/// A vertex buffer
///
pub struct IndexedTriangles {
    ///
    /// The OpenGL ID of the vertex buffer
    ///
    vertex_buffer_id: GLuint,
    ///
    /// The OpenGL ID of the vertex array
    ///
    vertex_array_id: GLuint,
    ///
    /// The OpenGL ID of the index buffer
    ///
    index_buffer_id: GLuint,
    ///
    /// The number of vertices in the buffer
    ///
    len: usize,
}

impl IndexedTriangles {
    ///
    /// Creates a new vertex buffer
    ///
    pub fn new(values: Vec<f32>, vertex_len: usize, color_len: usize, indices: Vec<usize>) -> Result<IndexedTriangles, Error> {
	IndexedTriangles::validate(&values, vertex_len, color_len, &indices)?;

	let mut vertex_buffer_id: GLuint = 0;
	let mut vertex_array_id: GLuint = 0;
	let mut index_buffer_id: GLuint = 0;
	let step = ((vertex_len + color_len) * std::mem::size_of::<f32>()) as gl::types::GLint;
	unsafe {
	    gl::GenBuffers(1, &mut vertex_buffer_id);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);
	    gl::BufferData(
		gl::ARRAY_BUFFER,
		(values.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
		values.as_ptr() as * const gl::types::GLvoid,
		gl::STATIC_DRAW
	    );
	    gl::GenVertexArrays(1, &mut vertex_array_id);
	    gl::BindVertexArray(vertex_array_id);
	    gl::EnableVertexAttribArray(0);
	    gl::VertexAttribPointer(0, vertex_len as gl::types::GLint, gl::FLOAT, gl::FALSE, step, std::ptr::null());
	    if color_len != 0 {
		gl::EnableVertexAttribArray(1);
		gl::VertexAttribPointer(
		    1,
		    color_len as gl::types::GLint,
		    gl::FLOAT,
		    gl::FALSE,
		    step,
		    (color_len * std::mem::size_of::<f32>()) as * const gl::types::GLvoid
		);		
	    }
	    gl::GenBuffers(1, &mut index_buffer_id);
	    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
	    gl::BufferData(
		gl::ELEMENT_ARRAY_BUFFER,
		(indices.len() * std::mem::size_of::<usize>()) as gl::types::GLsizeiptr,
		indices.as_ptr() as * const gl::types::GLvoid,
		gl::STATIC_DRAW
	    );
	    gl::BindBuffer(gl::ARRAY_BUFFER, 0 as gl::types::GLuint);
	    gl::BindVertexArray(0 as gl::types::GLuint);
	}
	Ok(IndexedTriangles {
	    vertex_buffer_id,
	    vertex_array_id,
	    index_buffer_id,
	    len: indices.len(),
	})
    }

    ///
    /// Draws the buffer
    ///
    pub fn draw(&self) {
	unsafe {
	    gl::BindVertexArray(self.vertex_array_id);
	    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer_id);
	    gl::DrawElements(gl::TRIANGLES, self.len as gl::types::GLsizei, gl::UNSIGNED_INT, 0 as * const gl::types::GLvoid);
	}
    }
    
    ///
    /// Validates the input
    ///
    fn validate(values: &Vec<f32>, vertex_len: usize, color_len: usize, indices: &Vec<usize>) -> Result<(), Error>{
	let step = color_len + vertex_len;
	if values.len() % step != 0 {
	    Err(Error::BadCoordinateCount)
	} else if indices.len() % 3 != 0 {
	    Err(Error::BadIndexCount)
	} else if indices.iter().any(|index| index * step >= values.len()){
	    Err(Error::BadIndex)
	} else {
	    Ok(())
	}
    }
}

impl Drop for IndexedTriangles {
    ///
    /// Destroys the OpenGL resources attached to this buffer
    ///
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteBuffers(1, &self.index_buffer_id as * const gl::types::GLuint);
	    gl::DeleteBuffers(1, &self.vertex_buffer_id as * const gl::types::GLuint);
	    gl::DeleteVertexArrays(1, &self.vertex_array_id as * const gl::types::GLuint);
	}
    }
}

///
/// Errors that can occur when using vertex buffers
///
#[derive(Debug)]
pub enum Error {
    ///
    /// Coordinate count does not match vertex buffer length
    ///
    BadCoordinateCount,
    ///
    /// Index count does not match the number of vertices
    ///
    BadIndexCount,
    ///
    /// An invalid index was found
    ///
    BadIndex,
}
