/*
Copyright 2020 Brandon Lucia <blucia@gmail.com>
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use crate::VtxTrait;
use crate::vertex::Vertex;


#[derive(Debug)]
pub struct GraphIter<'a, T: VtxTrait> {
  inner: &'a Graph<T>,
  cur: usize,
}

impl<'a, T: VtxTrait> Iterator for GraphIter<'a, T> {
  type Item = &'a Vertex<T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.cur < self.inner.vtxs.len() {
      let i: usize = self.cur;
      self.cur = self.cur + 1;
      Some(&*self.inner.vtxs[i])
    } else {
      None
    }
  }
}


/*TODO: Need to add in a vertex property array 
        separate from the graph structure arrays*/
#[derive(Debug)]
pub struct Graph<T: VtxTrait> {
  vtxs: Vec< Box< Vertex<T> > >,
  next_vtx: u64,
}


impl<T: VtxTrait> Graph< T> {
  pub fn new() -> Graph<T>{
    Graph{ vtxs: vec![], next_vtx: 0}
  } 

  pub fn iter<'a>(&'a self) -> GraphIter<'a, T> {
    GraphIter{ inner: self, cur: 0 }
  }

  pub fn add_vtx(&mut self, ind: u64, v: T){
    self.create_vtx( );
    self.init_vtx(ind, v); 
  }
 
  pub fn num_vtxs(&self) -> usize{
    self.vtxs.len()
  }
  
  pub fn get_vtx(&self, ind: usize) -> &Vertex<T>{
    &*(self.vtxs[ind])
  }

  pub fn set_vtx(&mut self, ind: usize, new_v: Vertex<T>){
    let vtxs = &mut self.vtxs[..];
    if let Some(v) = vtxs.get_mut(ind){
      *v = Box::new(new_v);
    }
  }
  
  pub fn add_edge(&mut self, ind: u64, nei: u64){
 
    if self.has_edge(ind,nei) == false {
      self.vtxs[ind as usize].add_neigh(nei);
    }

  }
 
  pub fn has_edge(&mut self, ind: u64, nei: u64) -> bool {  

    let v = self.get_vtx(ind as usize);
    match v{

      Vertex::V{ ref id, ref val, ref neigh} => {  

        for ne in neigh{
          if **ne == nei { return true; }
        }

      },
      Vertex::Empty => ()

    }
    return false;
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
