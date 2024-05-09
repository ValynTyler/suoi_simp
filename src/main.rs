use std::path::Path;

use suoi_simp::{Png, Resource};

fn main() {
    let path = Path::new("../assets/textures/monke.png");
    let _png = Png::import(path).expect("IMPORT_ERROR");

    

}
