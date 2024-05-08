use suoi_types::{Matrix, Matrix4};

#[test]
fn get_set() {
    let mut m = Matrix4::identity();
    m.set(0, 1, 69.0).unwrap();
    assert_eq!(m.get(0, 1).unwrap(), 69.0);
}

#[test]
fn transpose() {
    let mut m = Matrix4(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    );
    let n = m.clone();

    m.transpose();
    assert_eq!(
        m,
        Matrix4(
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        )
    );

    m.transpose();
    assert_eq!(n, m);
}

#[test]
fn identity_squared() {
    let mat = Matrix4::identity();

    assert_eq!(mat, &mat * &mat);
}
