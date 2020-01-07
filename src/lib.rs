pub trait VtxTrait: Ord + std::fmt::Debug + std::fmt::Display {}
impl<T> VtxTrait for T where T: Ord + std::fmt::Debug + std::fmt::Display {}

pub mod graph;
pub mod vertex;
