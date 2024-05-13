use suoi_types::Matrix4;

fn main() {
    let m = Matrix4(
        [6., 4., 2., 3.],
        [7., 42., 0., 0.],
        [0., 0., 9., 5.],
        [8., 1., 2., 4.],
    );

    println!("{}", m.minor(3, 1));
}
