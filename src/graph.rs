use crate::VtxTrait;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Graph<T: VtxTrait> {
  vtxs: Vec< Box< Vertex<T> > >,
  next_vtx: u64,
}


impl<T: VtxTrait> Graph< T> {
  pub fn new() -> Graph<T>{
    Graph{ vtxs: vec![], next_vtx: 0}
  } 

  pub fn add_vtx(&mut self, ind: u64, v: T){
    self.create_vtx( );
    self.init_vtx(ind, v); 
  }
  
  pub fn add_edge(&mut self, ind: u64, nei: u64){
    self.vtxs[ind as usize].add_neigh(nei);
  }

  fn init_vtx(&mut self, ind: u64, v: T){
    self.vtxs[ind as usize].init(ind,v);
  }

  fn create_vtx(&mut self){
    
    /*Vertex ids are private and increment with each created vtx*/
    let next_id = self.next_vtx;

    self.next_vtx = self.next_vtx + 1;

    self.vtxs.insert(next_id as usize, Box::new(Vertex::Empty) );

  }

  pub fn print(&self){
    for i in 0..self.vtxs.len() { self.vtxs[i].print(); }
  }

}/*impl Graph*/
