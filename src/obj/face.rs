use nerd::vector::{Vector2, Vector3};

#[allow(dead_code)]
#[derive(Debug)]
pub struct FaceElement {
    vertex_index: u32,
    normal_index: u32,
    uv_index: u32,
}

impl FaceElement {
    /**
    `FaceElement::new`
    ---
    Creates a new `FaceElement` from the supplied `vertex`, `normal`, and `uv` indices 
    */
    pub fn new(vertex_index: u32, normal_index: u32, uv_index: u32) -> Self {
        Self {
            vertex_index,
            normal_index,
            uv_index,
        }
    }

    // Immutable getter for `vertex_index` of `FaceElement`
    pub fn vertex_index(&self) -> u32 {
        self.vertex_index
    }

    // Immutable getter for `normal_index` of `FaceElement`
    pub fn normal_index(&self) -> u32 {
        self.vertex_index
    }

    // Immutable getter for `uv_index` of `FaceElement`
    pub fn uv_index(&self) -> u32 {
        self.vertex_index
    }
    
    /**
    `FaceElement::parse`
    ---
    Takes in a `face_string` of format: `vert_idx`/`uv_idx`/`norm_idx`
    and returns the constructed `FaceElement`, replacing syntax errors
    with the default `0` value.
    */
    pub fn parse(element_string: &str) -> Self {
        let tokens: Vec<u32> = element_string
            .split('/')
            .map(|token| match token.parse::<u32>() {
                Ok(num) => num,
                Err(_) => 0,
            })
            .collect();

        let vertex_index: u32 = *tokens.get(0).or(Some(&0)).unwrap();
        let uv_index: u32 = *tokens.get(1).or(Some(&0)).unwrap();
        let normal_index: u32 = *tokens.get(2).or(Some(&0)).unwrap();

        Self {
            vertex_index,
            normal_index,
            uv_index,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Face {
    elements: Vec<FaceElement>
}

impl Face {
    // `Face::new`
    // ---
    // Creates and returns a new instance of `Face`
    // using the supplied `FaceElement` list
    pub fn new(elements: Vec<FaceElement>) -> Self {
        Self {
            elements
        }
    }

    // Immutable getter for `Face.elements`
    pub fn elements(&self) -> &Vec<FaceElement> {
        &self.elements
    }

    pub fn read_vertices(&self, vertices: &Vec<Vector3>) -> Vec<Vector3> {
        let mut v = vec![];
        
        for elem in self.elements() {
            v.push(vertices[elem.vertex_index as usize - 1])
        }

        v
    }

    pub fn read_normals(&self, normals: &Vec<Vector3>) -> Vec<Vector3> {
        let mut v = vec![];
        
        for elem in self.elements() {
            v.push(normals[elem.normal_index as usize - 1])
        }

        v
    }

    pub fn read_uvs(&self, uvs: &Vec<Vector2>) -> Vec<Vector2> {
        let mut v = vec![];
        
        for elem in self.elements() {
            v.push(uvs[elem.uv_index as usize - 1])
        }

        v
    }
}
