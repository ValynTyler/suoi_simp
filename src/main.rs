use std::path::Path;

use suoi_simp::{Mtl, Obj, Resource};

fn main() {
    let obj_path = Path::new("../assets/models/cube.obj");
    let obj: Obj = Obj::import(obj_path).expect("ImportError");
    let mesh = obj.mesh();

    println!("{:#?}", mesh);
    
    let mtl_path = Path::new("../assets/models/cube.mtl");
    let _material = Mtl::import(mtl_path).expect("IMPORT_ERROR");
}
