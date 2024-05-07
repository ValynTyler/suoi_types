use nerd::{matrix::Matrix4, vector::{Vector, Vector3, Vector4}};
use suoi_types::{Angle::Deg, Quaternion};

fn main() {
    let axis = Vector3::new(0., 1., 0.).normalize();
    let angle = Deg(90.);

    let v = Vector3::new(1., 0., 0.);
    let q = Quaternion::axis_angle(axis, angle);
    let r: Matrix4 = q.into();

    println!("{:?}", q);
    println!("{:?}", v);
    println!("{:?}", &r * Vector4::from(v.into()));
}
