use std::path::Path;

use suoi_simp::obj::Obj;

fn main() {
    let path = Path::new("../suoi_assets/models/cube.obj");
    assert_eq!(path.extension().unwrap(), "obj");

    let mesh = Obj::import(path).expect("ImportError");
    println!("{:#?}", mesh);
}
