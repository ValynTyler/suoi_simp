use suoi_simp::{Obj, Resource};
use suoi_simp::Path;

fn main() {
    let obj_path = Path::new("../assets/models/stuff.obj");
    let obj: Obj = Obj::import(obj_path).expect("ImportError");

    let mesh = obj.meshes().last().unwrap();
    println!("{}", mesh.min_pos_index());
}
