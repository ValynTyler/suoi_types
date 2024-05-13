use suoi_types::Matrix4;
use suoi_types::Matrix;

fn main() {
    let m = Matrix4(
        [1., 4., 2., 3.],
        [2., 42., 0., 0.],
        [3., 7., 1., 5.],
        [4., 0., 9., 5.],
    );

    println!("{}", m.inverse());
}
