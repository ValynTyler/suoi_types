use suoi_types::Matrix4;
use suoi_types::Matrix;

fn main() {
    let mut m = Matrix4(
        [1., 4., 2., 3.],
        [2., 1., 0., 2.],
        [0., 0., 1., 2.],
        [0., 0., 0., 1.],
    );

    m.add_row_mul(0, 1, -2.0);

    println!("{}", m);
}
