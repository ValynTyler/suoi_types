use suoi_types::{Matrix, Matrix4};

fn main() {
    let mut m = Matrix4(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    );

    println!("{}", m);
    m.transpose();
    println!("{}", m);
}
