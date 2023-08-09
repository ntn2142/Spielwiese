use super::node;

use std::ptr;
type t1 = VoxelOctree<u64>;
struct VoxelOctree<V> {
    root: Option<node::Root<V>>,
    depth: u8,
}
impl<V> Drop for VoxelOctree<V> {
    fn drop(&mut self) {
        todo!()
    }
}
