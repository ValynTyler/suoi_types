use nerd::vector::Vector3;
use suoi_types::{Deg, Quaternion, Rotate};

fn main() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(10.));

    println!("{}", q.mat());
    println!("{}", Vector3::FORWARD.to_owned().rotate(q));
}
