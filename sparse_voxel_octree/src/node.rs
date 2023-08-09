use super::path::Octant;

use std::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::NonNull,
};

pub struct Node<V> {
    val: Option<V>,
    parent: Option<(NonNull<Node<V>>, Octant)>,
    children: Option<Box<[Node<V>; 8]>>,
}
impl<V> Node<V> {
    fn empty() -> Self {
        Self {
            val: None,
            parent: None,
            children: None,
        }
    }

    fn boxed_node() -> NonNull<Self> {
        NonNull::new(Box::leak(Box::new(Self::empty()))).unwrap()
    }
}
impl<V: Clone> Node<V> {
    fn init_children(&mut self) {
        let mut twig: [MaybeUninit<Node<V>>; 8] = unsafe { MaybeUninit::uninit().assume_init() };
        for (idx, elem) in twig.iter_mut().enumerate() {
            elem.write(Node {
                val: self.val.clone(),
                parent: Some((NonNull::new(self as *mut _).unwrap(), Octant::new(idx))),
                children: None,
            });
        }
        self.children = Some(Box::new(unsafe { mem::transmute_copy(&twig) }));
        mem::forget(twig)
    }
}

pub mod marker {
    pub struct Owned;
}
pub struct NodeRef<BorrowType, V> {
    pub node: NonNull<Node<V>>,
    pub _marker: PhantomData<BorrowType>,
}

pub type Root<V> = NodeRef<marker::Owned, V>;

impl<V> NodeRef<marker::Owned, V> {
    fn new() -> Self {
        Self {
            node: Node::boxed_node(),
            _marker: PhantomData,
        }
    }
}
