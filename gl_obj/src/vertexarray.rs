use super::*;
use gl_safe::*;

pub trait Any {}

pub struct VertexArray {
	handle: GLuint,
	len: usize,
	keepalive: Vec<Buffer>, // owned GL data, dropped together with self
}

impl VertexArray {
	/// Create a vertex array object.
	/// http://docs.gl/gl4/glCreateVertexArrays
	pub fn create() -> Self {
		let handle = glCreateVertexArray();
		//debug!("create VertexArray {}", handle);
		Self {
			handle,
			len: 0,
			keepalive: Vec::new(),
		}
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}

	/// Number of vertices in the array.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Enable a generic vertex attribute array.
	/// http://docs.gl/gl4/glEnableVertexAttribArray
	pub fn enable_attrib(self, index: u32) -> Self {
		glEnableVertexArrayAttrib(self.handle, index);
		self
	}

	/// Associate a vertex attribute and a vertex buffer binding for a vertex array object
	/// http://docs.gl/gl4/glVertexAttribBinding
	pub fn attrib_binding(self, attribindex: u32, bindingindex: u32) -> Self {
		glVertexArrayAttribBinding(self.handle, attribindex, bindingindex);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_format(self, attribindex: u32, size: i32, typ: GLenum, normalized: bool, relativeoffset: u32) -> Self {
		glVertexArrayAttribFormat(self.handle, attribindex, size, typ, normalized, relativeoffset);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_iformat(self, attribindex: u32, size: i32, typ: GLenum, relativeoffset: u32) -> Self {
		glVertexArrayAttribIFormat(self.handle, attribindex, size, typ, relativeoffset);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_lformat(self, attribindex: u32, size: i32, typ: GLenum, relativeoffset: u32) -> Self {
		glVertexArrayAttribLFormat(self.handle, attribindex, size, typ, relativeoffset);
		self
	}

	/// Bind a buffer to a vertex buffer bind point.
	/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml
	pub fn vertex_buffer(mut self, bindingindex: GLuint, buffer: Buffer, offset: GLintptr, stride: GLsizei) -> Self {
		if self.len == 0 {
			self.len = buffer.len()
		}
		debug_assert!(self.len == buffer.len()); // subsequent calls (e.g.: set texture coordinates after setting positions) should apply to same number of vertices
		glVertexArrayVertexBuffer(self.handle, bindingindex, buffer.handle(), offset, stride);
		self.keepalive.push(buffer);
		self
	}

	/// Bind a buffer to a vertex buffer bind point.
	/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml
	#[allow(non_snake_case)]
	pub fn bind(&self) {
		glBindVertexArray(self.handle)
	}
}

impl Drop for VertexArray {
	fn drop(&mut self) {
		//debug!("drop VertexArray {}", self.handle);
		glDeleteVertexArray(self.handle);
	}
}
