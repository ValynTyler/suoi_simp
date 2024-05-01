use suoi_simp::{Obj, Resource};
use suoi_simp::Path;

fn main() {
    let obj_path = Path::new("../assets/models/stuff.obj");
    let _obj: Obj = Obj::import(obj_path).expect("ImportError");

    // for mesh in _obj.meshes() {
    //     println!("{}", mesh.get_name());
    //     for face in mesh.faces() {
    //         println!("{:#?}", face);
    //     }
    // }
}
