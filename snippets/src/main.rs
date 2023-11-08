use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node<V> {
    val: V,
    parent: Option<(NonNull<Node<V>>, u8)>,
    children: Option<Box<[Option<Node<V>>; 8]>>,
}

fn main() {
    let tmp:[Option<Node<u64>>;8] = Default::default();
    println!("{:?}", tmp);
}