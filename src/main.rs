use suoi_types::Quaternion;

fn main() {
    let q = Quaternion::new(0.707, 0.5, 0.5, 0.0);
    
    println!("{}", q);
    println!("{}", q.mat());
}
