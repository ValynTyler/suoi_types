use nerd::{matrix::{Matrix, Matrix4}, vector::{Vector, Vector3}};
use suoi_types::{Angle, Deg, Quaternion, Rotate};

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
fn angle_axis_90() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(90.));
    assert_eq!(q, Quaternion::new(0.70710677, 0.0, 0.70710677, 0.0));
}

#[test]
fn recip_90() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(90.));
    assert_eq!(
        q.recip(),
        Quaternion::new(0.7071068, -0.0, -0.7071068, -0.0)
    );
}

#[test]
#[rustfmt::skip]
fn mat_45() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(45.));

    assert_eq!(q.mat(), Matrix4([
         0.7071067, 0.0, 0.7071068, 0.0,
         0.0,       1.0, 0.0,       0.0,
        -0.7071068, 0.0, 0.7071067, 0.0,
         0.0,       0.0, 0.0,       1.0,
    ])
    )
}

#[test]
#[rustfmt::skip]
fn mat_60() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(60.));

    assert_eq!(q.mat(), Matrix4([
         0.5,           0.0, 0.8660254, 0.0,
         0.0,           1.0, 0.0,       0.0,
         -0.8660254,    0.0, 0.5,       0.0,
         0.0,           0.0, 0.0,       1.0,
    ])
    )
}

#[test]
fn fwd_column() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(10.));
    
    let mat = q.mat();
    let fwd = Vector3 {
        x: mat.get(1, 3),
        y: mat.get(2, 3),
        z: mat.get(3, 3),
    };
    assert!(
        (fwd - Vector3::FORWARD.to_owned().rotate(q)).len() < 0.0001
    );
}
