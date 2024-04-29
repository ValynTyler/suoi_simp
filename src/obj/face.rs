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
        let normal_index: u32 = *tokens.get(1).or(Some(&0)).unwrap();
        let uv_index: u32 = *tokens.get(2).or(Some(&0)).unwrap();

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
    pub fn new(elements: Vec<FaceElement>) -> Self {
        Self {
            elements
        }
    }
}
