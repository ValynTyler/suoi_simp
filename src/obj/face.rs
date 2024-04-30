/**
`obj::FaceELement`
---
Substructure for `obj::Face` that stores the
index into the `position`, `normal`, and `uv` buffers
of a single polygonal `vertex` 
*/
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

    /// Immutable getter for `vertex_index` of `FaceElement`
    pub fn vertex_index(&self) -> u32 {
        self.vertex_index
    }

    /// Immutable getter for `normal_index` of `FaceElement`
    pub fn normal_index(&self) -> u32 {
        self.normal_index
    }

    /// Immutable getter for `uv_index` of `FaceElement`
    pub fn uv_index(&self) -> u32 {
        self.uv_index
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

/**
`obj::Face`
---
Represents a polygonal face by storing indices
for the relevant `position`, `normal` and `uv` buffers
*/
#[derive(Debug)]
pub struct Face {
    elements: Vec<FaceElement>
}

impl Face {
    /// `Face::new`
    /// ---
    /// Creates and returns a new instance of `Face`
    /// using the supplied `FaceElement` list
    pub fn new(elements: Vec<FaceElement>) -> Self {
        Self {
            elements
        }
    }

    /// Immutable getter for `Face.elements`
    pub fn elements(&self) -> &Vec<FaceElement> {
        &self.elements
    }
}
