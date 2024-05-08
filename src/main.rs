use suoi_types::{Matrix, Matrix4};

fn main() {
    let mut m = Matrix4(
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [1., 1., 0., 1.],
    );
    m.transpose();
    m.set(0, 1, 69.0);
    println!("{}", m);
}
