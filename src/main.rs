use std::path::Path;

use suoi_simp::obj::Obj;

fn main() {
    let path = Path::new("assets/models/cube/cube.obj");
    assert_eq!(path.extension().unwrap(), "obj");

    let mesh = Obj::import(path).expect("Error");
    println!("{:#?}", mesh);
}
