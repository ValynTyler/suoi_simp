use nerd::vector::{Vector2, Vector3};

use crate::{face::Face, MtlMaterial};

#[allow(unused)]
#[derive(Debug)]
pub struct ObjMesh {
    // token: usemtl
    active_material: MtlMaterial,

    // token: o
    name: String,

    // tokens: v, vn, vt
    vertex_data: Vec<Vector3>,
    normal_data: Vec<Vector3>,
    uv_data: Vec<Vector2>,

    // token: f
    face_data: Vec<Face>,
}

impl ObjMesh {
    /**
    `ObjMesh::empty`
    ---
    Creates an empty instance of `ObjMesh`
    */
    pub fn empty() -> Self {
        Self {
            active_material: MtlMaterial::empty(),
            name: String::new(),
            vertex_data: vec![],
            normal_data: vec![],
            uv_data: vec![],
            face_data: vec![],
        }
    }

    /// Immutable getter for `vertex_data` of `ObjMesh`
    pub fn positions(&self) -> &Vec<Vector3> {
        &self.vertex_data
    }

    /// Immutable getter for `normal_data` of `ObjMesh`
    pub fn normals(&self) -> &Vec<Vector3> {
        &self.normal_data
    }

    /// Immutable getter for `uv_data` of `ObjMesh`
    pub fn uvs(&self) -> &Vec<Vector2> {
        &self.uv_data
    }

    /// Immutable getter for `face_data` of `ObjMesh`
    pub fn faces(&self) -> &Vec<Face> {
        &self.face_data
    }

    /// Mutable getter for `material` of `ObjMesh`
    pub fn material(&mut self) -> &mut MtlMaterial {
        &mut self.active_material
    }

    pub fn get_material(&self) -> &MtlMaterial {
        &self.active_material
    }

    /**
    `ObjMesh.set_name`
    ---
    Sets `name` field of `self`
    */
    pub fn set_name(&mut self, value: &str) {
        self.name = value.to_owned()
    }

    /**
    `ObjMesh.load_vertex`
    ---
    Loads a `Vector3` into the `vertex_data` buffer of `self`
    */
    pub fn load_vertex(&mut self, value: Vector3) {
        self.vertex_data.push(value)
    }

    /**
    `ObjMesh.load_normal`
    ---
    Loads a `Vector3` into the `normal_data` buffer of `self`
    */
    pub fn load_normal(&mut self, value: Vector3) {
        self.normal_data.push(value)
    }

    /**
    `ObjMesh.load_uv`
    ---
    Loads a `Vector2` into the `uv_data` buffer of `self`
    */
    pub fn load_uv(&mut self, value: Vector2) {
        self.uv_data.push(value)
    }

    /**
    `ObjMesh.load_face`
    ---
    Loads a `Face` into the `face_data` buffer of `self`
    */
    pub fn load_face(&mut self, face: Face) {
        self.face_data.push(face);
    }
}