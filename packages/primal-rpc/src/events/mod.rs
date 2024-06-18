use crate::v1::types::Package;

pub mod tags;

pub enum DfsPostorderEvent<T> {
    /// A leaf node
    Leaf { tag: T },
    /// A branch node
    Branch { tag: T, child_count: usize },
}

pub struct EventGenerationContext {
    pub u32: Vec<u32>,
    pub strings: Vec<String>,

}