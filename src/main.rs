use suoi_types::Quaternion;

fn main() {
    let q = Quaternion {
        a: 1.0,
        b: 2.0,
        c: 3.0,
        d: 4.0
    };

    let r = Quaternion {
        a: 1.0,
        b: 2.0,
        c: 3.0,
        d: 5.0
    };

    println!("{:?}", q);
    println!("{:?}", r);
    println!("{:?}", q * r);
    println!("{:?}", r * q);
}
