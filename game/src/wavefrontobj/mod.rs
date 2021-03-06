use crate::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Parse a Wavefront OBJ file. See
/// https://en.wikipedia.org/wiki/Wavefront_.obj_file
pub fn parse_file(fname: &Path) -> Result<MeshBuffer> {
	println!("loading {}", fname.to_string_lossy());
	parse(File::open(fname)?)
}

/// Like parse_file, but accepts a reader.
pub fn parse<R: Read>(r: R) -> Result<MeshBuffer> {
	Parser::new().parse(r)
}

struct Parser {
	curr_line: u32, // current line, for error messages
	obj_start: u32,
	curr_mtl: String,

	v_pos: Vec<vec3>,    // vertex positions, parsed from "v ..."
	t: Vec<vec2>,        // texture coordinates, parsed from "t ..."
	n: Vec<vec3>,        // normals, parsed from "n ..."
	f_def: Vec<FaceDef>, // faces, parsed from "f ..."
	calc_n: Vec<vec3>,   // calculated normals, for when they're not specified
	objects: Vec<ObjDef>,
}

// 3 or 4 elements.
#[derive(Default)]
struct FaceDef([FaceVert; 4]);

#[derive(Default)]
struct FaceVert {
	v: u32,
	n: u32,
	t: u32,
}

struct ObjDef {
	//mtl: String,
	face_range: (u32, u32),
}

impl Parser {
	fn new() -> Self {
		Self {
			curr_line: 0,
			obj_start: 0,
			curr_mtl: String::new(),

			v_pos: Vec::new(),
			f_def: Vec::new(),
			t: Vec::new(),
			n: Vec::new(),
			calc_n: Vec::new(),
			objects: Vec::new(),
		}
	}

	//fn fix_blender(v: vec3) -> vec3 {
	//	vec3(v.x, v.z, v.y)
	//}

	fn parse<R: Read>(mut self, r: R) -> Result<MeshBuffer> {
		let reader = BufReader::new(r);
		for line in reader.lines() {
			// on parse error, prefix with current line number
			if let Err(e) = &self.parse_line(line?) {
				return err(format!("line {}: {}", self.curr_line, e));
			}
		}

		self.push_curr_obj();
		self.calc_normals()?;

		let mut b = MeshBuffer::new();
		for obj in &self.objects {
			for i in obj.face_range.0..obj.face_range.1 {
				let face = self.face(i);
				let fvertices = face.vertices();

				if face.is_quad() {
					// triangulate
					let v0 = self.face_vertex(&fvertices[0])?;
					let v1 = self.face_vertex(&fvertices[1])?;
					let v2 = self.face_vertex(&fvertices[2])?;
					let v3 = self.face_vertex(&fvertices[3])?;
					b.push_all(&[&v0, &v1, &v2, &v2, &v3, &v0]);
				} else {
					let v0 = self.face_vertex(&fvertices[0])?;
					let v1 = self.face_vertex(&fvertices[1])?;
					let v2 = self.face_vertex(&fvertices[2])?;
					b.push_all(&[&v0, &v1, &v2]);
				}
			}
		}
		Ok(b)
	}

	fn face(&self, i: u32) -> &FaceDef {
		&self.f_def[i as usize]
	}

	fn face_vertex(&self, vdef: &FaceVert) -> Result<Vertex> {
		let texcoord = self.tex_coord(vdef.t)?;
		let texcoord = vec2(texcoord.x, 1.0 - texcoord.y); // blender-compatible
		Ok(Vertex {
			pos: self.vertex_pos(vdef.v)?,
			texcoord,
			attrib: self.normal(vdef)?,
		})
	}

	// Get the i'th vertex with base-1 indexing.
	// Return a nice error if the index is out-of-bounds.
	fn vertex_pos(&self, i: u32) -> Result<vec3> {
		Self::index_base1("vertex", &self.v_pos, i)
	}

	// Get the i'th normal with base-1 indexing.
	// Return a nice error if the index is out-of-bounds.
	fn normal(&self, fv: &FaceVert) -> Result<vec3> {
		if fv.n == 0 {
			Ok(self.calc_n[(fv.v - 1) as usize])
		} else {
			Self::index_base1("normal", &self.n, fv.n)
		}
	}

	// Get the i'th texture coordinates with base-1 indexing.
	// Return a nice error if the index is out-of-bounds.
	fn tex_coord(&self, i: u32) -> Result<vec2> {
		if i == 0 {
			return Ok(vec2(0.0, 0.0));
		}
		Self::index_base1("texture coordinates", &self.t, i)
	}

	fn parse_line(&mut self, line: String) -> Result<()> {
		self.curr_line += 1;
		let mut fields = line.split_ascii_whitespace();
		let first = fields.next().unwrap_or_default();
		let args: Vec<&str> = fields.collect();

		match first {
			"" => Ok(()),  // empty line
			"#" => Ok(()), // comment
			"v" => self.parse_v(&args),
			"vt" => self.parse_t(&args),
			"vn" => self.parse_n(&args),
			"f" => self.parse_f(&args),
			"usemtl" => self.parse_usemtl(&args),
			_ => Ok(()), // ignore unknown commands
		}
	}

	// Geometric vertex
	// A vertex can be specified in a line starting with the letter v.
	// That is followed by (x,y,z[,w]) coordinates.
	// W is optional and defaults to 1.0.
	// Some applications support vertex colors, by putting red, green and blue values after x y and z.
	// The color values range from 0 to 1.[1]
	//
	//   # List of geometric vertices, with (x, y, z [,w]) coordinates, w is optional and defaults to 1.0.
	//   v 0.123 0.234 0.345 1.0
	//   v ...
	#[must_use]
	fn parse_v(&mut self, args: &[&str]) -> Result<()> {
		// TODO: if a 4th coordinate is present, divide by it
		if !(args.len() == 3) {
			return err(format!("vertex: need 3 coordinates, have: {:?}", args));
		}

		self.v_pos.push(vec3(args[0].parse()?, args[1].parse()?, args[2].parse()?));

		Ok(())
	}

	// Parse a line with texture coordinates, and store the result.
	// # List of texture coordinates, in (u, [,v ,w]) coordinates, these will vary between 0 and 1. v, w are optional and default to 0.
	// vt 0.500 1 [0]
	// vt ...
	#[must_use]
	fn parse_t(&mut self, args: &[&str]) -> Result<()> {
		// If a 3th coordinate is present, ignore it.
		if !(args.len() == 2 || args.len() == 3) {
			return err(format!("vt: need 2 or 3 coordinates, have: {:?}", args));
		}

		self.t.push(vec2(args[0].parse()?, args[1].parse()?));

		Ok(())
	}

	#[must_use]
	fn parse_n(&mut self, args: &[&str]) -> Result<()> {
		if args.len() != 3 {
			return err(format!("vn: need 3 coordinates, have: {:?}", args));
		}

		self.n.push(vec3(args[0].parse()?, args[1].parse()?, args[2].parse()?));

		Ok(())
	}

	// Faces are defined using lists of vertex, texture and normal indices in the format
	//
	//     vertex_index/texture_index/normal_index
	//
	// for which each index starts at 1 and increases corresponding to the order in which the referenced element was defined.
	// Polygons such as quadrilaterals can be defined by using more than three indices.
	//   # Polygonal face element
	//   f 1 2 3
	//   f 3/1 4/2 5/3
	//   f 6/4/1 3/5/3 7/6/5
	//   f 7//1 8//2 9//3
	//   f ...
	#[must_use]
	fn parse_f(&mut self, args: &[&str]) -> Result<()> {
		if !(args.len() == 3 || args.len() == 4) {
			return err(format!("need 3 or 4 face indices, got {:?}", args));
			//return Ok(()); //TODO: subdivide?
			//return error(format!("need 3 or 4 face indices, got {:?}", args));
		}

		let mut fdef = FaceDef::default();
		for (i, arg) in args.iter().enumerate() {
			fdef.0[i] = Self::parse_f_arg(arg)?;
		}
		self.f_def.push(fdef);

		Ok(())
	}

	// parse a single face vertex, like `1/2/3`, `1//2`, `1`.
	fn parse_f_arg(arg: &str) -> Result<FaceVert> {
		let words: Vec<&str> = arg.split("/").collect();
		if words.len() > 3 {
			return err(format!("face: too many /'s: {:?}", words));
		}

		let v: u32 = words[0].parse()?;

		let t: u32 = if words.len() > 1 && words[1] != "" { words[1].parse()? } else { 0 };

		let n: u32 = if words.len() > 2 { words[2].parse()? } else { 0 };

		Ok(FaceVert { v, t, n })
	}

	#[must_use]
	fn parse_usemtl(&mut self, args: &[&str]) -> Result<()> {
		if args.len() != 1 {
			return err(format!("usemtl: need 1 argument, got: {:?}", args));
		}

		if self.f_def.len() != 0 {
			self.push_curr_obj();
		}

		self.curr_mtl = args[0].to_owned();
		self.obj_start = self.f_def.len() as u32;
		Ok(())
	}

	fn push_curr_obj(&mut self) {
		let start = self.obj_start;
		let end = self.f_def.len() as u32;

		// Some valid OBJ files have a "usemtl" not followed by any faces.
		// So don't attempt to push an "empty" object.
		if start != end {
			self.objects.push(ObjDef {
				//mtl: self.curr_mtl.clone(),
				face_range: (start, end),
			});
		}
	}

	// calculate normals, if not specified.
	#[must_use]
	fn calc_normals(&mut self) -> Result<()> {
		// only calculate if needed
		if self.has_all_normals() {
			return Ok(());
		}

		// allocate normals, all set to (0, 0, 0).
		// will add gemotric normals of all adjacent faces.
		self.calc_n = Vec::with_capacity(self.v_pos.len());
		for _ in &self.v_pos {
			self.calc_n.push(vec3::ZERO);
		}

		// calculate normals: add face geometric normal
		// to the normals belonging to each vertex the face shares.
		for fdef in &self.f_def {
			let vdef = fdef.vertices();
			if fdef.is_quad() {
				for i in 0..4 {
					let a = self.vertex_pos(vdef[(i + 0) % 3].v)?;
					let b = self.vertex_pos(vdef[(i + 1) % 3].v)?;
					let c = self.vertex_pos(vdef[(i + 2) % 3].v)?;
					let n = triangle_normal(a, b, c);
					self.calc_n[(vdef[i].v - 1) as usize] += n;
				}
			} else {
				for i in 0..3 {
					let a = self.vertex_pos(vdef[i].v)?;
					let b = self.vertex_pos(vdef[(i + 1) % 3].v)?;
					let c = self.vertex_pos(vdef[(i + 2) % 3].v)?;
					let n = triangle_normal(a, b, c);
					self.calc_n[(vdef[i].v - 1) as usize] += n;
				}
			}
		}

		// calc_n[i] now holds the sum of gemotric nomrmals
		// of all faces sharing vertex[i]. it needs to be normalized.
		for n in &mut self.calc_n {
			n.normalize()
		}

		Ok(())
	}

	// are all normals specified?
	// if not, they will need to be calculated.
	fn has_all_normals(&self) -> bool {
		for fdef in &self.f_def {
			if !fdef.has_normals() {
				return false;
			}
		}
		true
	}

	// Index a vector in base-1.
	// Return a nice error if the index is out-of-bounds.
	fn index_base1<T: Clone>(descr: &str, v: &Vec<T>, i1: u32) -> Result<T> {
		assert!(i1 != 0);
		let i1 = i1 as usize;
		let i = i1 - 1; // 1-based indexing
		if i1 >= 1 && i < v.len() {
			Ok(v[i].clone())
		} else {
			err(format!("invalid {} index: {} (max = {})", descr, i1, v.len()))
		}
	}
}

impl FaceDef {
	// returns 3 or 4 vertices, depending on how many were specified
	fn vertices(&self) -> &[FaceVert] {
		if self.is_quad() {
			&self.0
		} else {
			&self.0[0..3]
		}
	}

	// does this face have 4 vertices?
	fn is_quad(&self) -> bool {
		self.0[3].v != 0
	}

	// does this face have all its normals specified?
	// (if not, they will need to be calculated)
	fn has_normals(&self) -> bool {
		for v in self.vertices() {
			if v.n == 0 {
				return false;
			}
		}
		true
	}
}

fn triangle_normal(a: vec3, b: vec3, c: vec3) -> vec3 {
	(b - a).cross(c - a)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		let input = r"
 # test mesh
 v 1 2 3
 v 2 3 4
 v 3 4 5

 vt 1 1
 vn 1 0 0

 f 1 2 3
 f 1/1 2//1 3/1/1
 "
		.as_bytes();
		let _ = parse(input).unwrap(); // test passes if it does not error out
	}
}
