use suoi_types::{Deg, Matrix, Matrix4, Vector3};

fn main() {
    let m = Matrix4::rotate_around(Vector3::new(1., 0., 0.), Deg(45.0));

    println!("{}", m.det());
}
