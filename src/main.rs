mod foil;
mod impl_data;

use foil::*;
use impl_data::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    match Foil::import("work/MH45.dat") {
        Ok(f) => {
            println!("{}", f.name);
            export_p2_vec("work/upper.csv", &f.upp_ps);
            export_p2_vec("work/lower.csv", &f.low_ps);
            export_p2_vec("work/mid.csv", &f.mid_ps);
        }
        Err(s) => println!("{}", s),
    };
}

fn export_p2_vec(filename: &str, vec: &Vec<Point2D>) {
    let mut file = File::create(filename).unwrap();
    for p in vec {
        file.write(format!("{}, {}\n", p.x, p.z).as_bytes())
            .unwrap();
    }
}
