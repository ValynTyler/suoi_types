use nerd::vector::Vector3;
use suoi_types::{Deg, Quaternion};

fn main() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(45.));

    println!("{}", q);
    println!("{}", q.mat());
}
