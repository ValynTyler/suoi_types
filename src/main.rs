use nerd::{
    matrix::Matrix4,
    vector::{Vector, Vector3, Vector4},
};
use suoi_types::{
    Angle::{self, Deg},
    Quaternion, Transform,
};

fn main() {
    let mut t = Transform::default();

    let axis = Vector3::UP;
    let angle = Angle::Deg(90.0);
    // t.set_rotation(Quaternion::axis_angle(axis, angle));

    println!("{}", t.rotation().mat());
}
