use nerd::vector::{Vector2, Vector3};

use crate::{face::Face, MtlMaterial};

#[allow(unused)]
#[derive(Debug)]
pub struct ObjMesh {
    // token: o
    name: String,

    // tokens: v, vn, vt
    position_data: Vec<Vector3>,
    normal_data: Vec<Vector3>,
    uv_data: Vec<Vector2>,
    // token: f
    face_data: Vec<Face>,

    // token: usemtl
    active_material: MtlMaterial,
}

impl ObjMesh {
    /**
    `ObjMesh::empty`
    ---
    Creates an empty instance of `ObjMesh`
    */
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            position_data: vec![],
            normal_data: vec![],
            uv_data: vec![],
            face_data: vec![],
            active_material: MtlMaterial::empty(),
        }
    }

    /// Immutable getter for `position_data` of `ObjMesh`
    pub fn positions(&self) -> &Vec<Vector3> {
        &self.position_data
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

    /**
    `ObjMesh.set_name`
    ---
    Sets `name` field of `self`
    */
    pub fn set_name(&mut self, value: &str) {
        self.name = value.to_owned()
    }
    
    /// Getter for `material` of `ObjMesh`
    pub fn get_material(&self) -> &MtlMaterial {
        &self.active_material
    }
    
    /// Setter for `material` of `ObjMesh`
    pub fn set_material(&mut self, value: MtlMaterial) {
        self.active_material = value;
    }

    /**
    `ObjMesh.load_vertex`
    ---
    Loads a `Vector3` into the `vertex_data` buffer of `self`
    */
    pub fn load_position(&mut self, value: Vector3) {
        self.position_data.push(value)
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
