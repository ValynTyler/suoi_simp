use suoi_simp::{Obj, Resource};
use suoi_simp::Path;

fn main() {
    let obj_path = Path::new("../assets/models/cube.obj");
    let obj: Obj = Obj::import(obj_path).expect("ImportError");

    println!("{:#?}", obj);
}
