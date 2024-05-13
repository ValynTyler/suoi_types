use suoi_types::{Matrix, Matrix4, Vector3};

#[test]
fn get_set() {
    let mut m = Matrix4::identity();
    m.set(0, 1, 69.0).unwrap();
    assert_eq!(m.get(0, 1).unwrap(), 69.0);
}

#[test]
fn transpose() {
    let m = Matrix4(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    );

    assert_eq!(
        m.transpose(),
        Matrix4(
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        )
    );
}

#[test]
fn identity_squared() {
    let mat = Matrix4::identity();

    assert_eq!(mat, &mat * &mat);
}

#[test]
fn scale_vector() {
    let v = Vector3 {
        x: 3.,
        y: 4.,
        z: 5.,
    };

    assert_eq!(
        Vector3::new(30.0, 40.0, 50.0),
        &Matrix4::uniform_scale(10.0) * v
    );
}

#[test]
fn row_swap() {
    let mut m = Matrix4(
        [1., 0., 0., 3.],
        [0., 1., 0., 2.],
        [0., 0., 1., 2.],
        [0., 0., 0., 1.],
    );

    m.swap_rows(0, 2);

    assert_eq!(
        m,
        Matrix4(
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 1.0, 0.0, 2.0],
            [1.0, 0.0, 0.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    );
}

#[test]
fn row_mul() {
    let mut m = Matrix4(
        [1., 4., 2., 3.],
        [0., 1., 0., 2.],
        [0., 0., 1., 2.],
        [0., 0., 0., 1.],
    );

    m.mul_row(0, 2.0);

    assert_eq!(
        m,
        Matrix4(
            [2.0, 8.0, 4.0, 6.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0]
        )
    );
}

#[test]
fn row_add_mul() {
    let mut m = Matrix4(
        [1., 4., 2., 3.],
        [2., 1., 0., 2.],
        [0., 0., 1., 2.],
        [0., 0., 0., 1.],
    );

    m.add_row_mul(0, 1, -2.0);

    assert_eq!(m, Matrix4(
        [1.0, 4.0, 2.0, 3.0],
        [0.0, -7.0, -4.0, -4.0],
        [0.0, 0.0, 1.0, 2.0],
        [0.0, 0.0, 0.0, 1.0],
    ));
}
