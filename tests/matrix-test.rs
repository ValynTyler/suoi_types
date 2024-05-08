use suoi_types::{Matrix, Matrix4};

#[test]
fn get_set() {
    let mut m = Matrix4::identity();
    m.set(0, 1, 69.0);
    assert_eq!(m.get(0, 1), 69.0);
}
