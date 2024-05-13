use suoi_types::Matrix4;
use suoi_types::Matrix;

fn main() {
    let m = Matrix4(
        [1., 0., 0., 3.],
        [0., 1., 0., 2.],
        [0., 0., 1., 2.],
        [0., 0., 0., 1.],
    );

    println!("{}", m.inverse());
}
