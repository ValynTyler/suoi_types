use suoi_types::{Matrix4, Vector3};

fn main() {
    let v = Vector3 {
        x: 3.,
        y: 4.,
        z: 5.,
    };

    println!("{}", &Matrix4::uniform_scale(10.0) * v);
}
