use suoi_types::{Angle, Deg, Matrix, Matrix4, Quaternion, Vector3, Vector};

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
    let q = Quaternion::axis_angle(Vector3::up(), Deg(90.));
    assert_eq!(q, Quaternion::new(0.70710677, 0.0, 0.70710677, 0.0));
}

#[test]
fn recip_90() {
    let q = Quaternion::axis_angle(Vector3::up(), Deg(90.));
    assert_eq!(
        q.recip(),
        Quaternion::new(0.7071068, -0.0, -0.7071068, -0.0)
    );
}

#[test]
#[rustfmt::skip]
fn mat_45() {
    let q = Quaternion::axis_angle(Vector3::up(), Deg(45.));

    assert_eq!(q.mat(), Matrix4(
        [ 0.7071067, 0.0, 0.7071068, 0.0],
        [ 0.0,       1.0, 0.0,       0.0],
        [-0.7071068, 0.0, 0.7071067, 0.0],
        [ 0.0,       0.0, 0.0,       1.0],
    ))
}

#[test]
#[rustfmt::skip]
fn mat_60() {
    let q = Quaternion::axis_angle(Vector3::up(), Deg(60.));

    assert_eq!(q.mat(), Matrix4(
        [ 0.5,           0.0, 0.8660254, 0.0],
        [ 0.0,           1.0, 0.0,       0.0],
        [ -0.8660254,    0.0, 0.5,       0.0],
        [ 0.0,           0.0, 0.0,       1.0],
    ))
}

#[test]
fn fwd_column() {
    let q = Quaternion::axis_angle(Vector3::up(), Deg(10.));

    let mat = q.mat();
    let fwd = Vector3 {
        x: mat.get(0, 2).unwrap(),
        y: mat.get(1, 2).unwrap(),
        z: mat.get(2, 2).unwrap(),
    };

    println!("{}", fwd);

    assert!((fwd - Vector3::fwd().to_owned().rotate(q)).len() < 0.0001);
}
