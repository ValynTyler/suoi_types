use nerd::vector::Vector3;
use suoi_types::{Deg, Quaternion};

fn main() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(10.));

    println!("{}", q.mat());
    println!("{}", Quaternion::from(q * Vector3::FORWARD) * q.recip());
}
