use nerd::{matrix::Matrix4, vector::Vector3};
use suoi_types::{Deg, Quaternion};

fn main() {
    let q = Quaternion::axis_angle(Vector3::UP, Deg(45.));

    println!("{}", q);
    println!("{}", q.recip());

    let (r, i, j, k) = (q.a, q.b, q.c, q.d);
    let r = Matrix4([
        1. - 2.*(j*j + k*k),    2.*(i*j - k*r),         2.*(i*k + j*r),         0.,
        2.*(i*j + k*r),         1. - 2.*(i*i + k*k),    2.*(j*k - i*r),         0.,
        2.*(i*k - j*r),         2.*(j*k + i*r),         1. - 2.*(i*i + j*j),    0.,
        0.,                     0.,                     0.,                     1.,
    ]);

    println!("{r}");
}
