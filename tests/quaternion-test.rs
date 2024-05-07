use nerd::vector::Vector3;
use suoi_types::{Angle, Deg, Quaternion};

#[test]
fn empty() {
    let phi = Deg(00.0).rad().0;
    let u = Vector3::new(1.0, 0.0, 0.0);

    let q = Quaternion::from(u * phi.sin()) + phi.cos();

    assert_eq!(q, Quaternion::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn rotate_30() {
    let phi = Deg(30.0).rad().0;
    let u = Vector3::new(1.0, 0.0, 0.0);

    let q = Quaternion::from(u * phi.sin()) + phi.cos();

    assert_eq!(q, Quaternion::new(0.8660254, 0.5, 0.0, 0.0));
}

#[test]
fn angle_axis_45() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(45.));
    assert_eq!(q, Quaternion::new(0.70710677, 0.0, 0.70710677, 0.0));
}

#[test]
fn recip_45() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(45.));
    assert_eq!(q.recip(), Quaternion::new(0.7071068, -0.0, -0.7071068, -0.0));
}
