use crate::VtxTrait;

#[derive(Debug)]
pub enum Vertex<T: VtxTrait> {
  V{
    id: u64,
    val: T,
    neigh: Vec<Box<u64>>,
  },
  Empty,
}

impl<T: VtxTrait> Vertex<T> {

  pub fn init(&mut self, new_id: u64, nv: T){
    match self{
      &mut Vertex::V{ .. } => {},
      &mut Vertex::Empty => {
        *self = Vertex::V{ id: new_id, val: nv, neigh: vec![] };
      }
    }
  }

  pub fn add_neigh(&mut self, n_id: u64){

    match self{

      &mut Vertex::V{ ref mut neigh, .. }  => {
        neigh.push(Box::new(n_id));
      },
      Vertex::Empty => return,

    }

  }

  pub fn print(&self){

    match self{

      Vertex::V{ ref id, ref val, ref neigh } => { 

        println!("{} => {}\n|----|",id,val); 

        for i in 0..neigh.len() { 
          println!("{}",neigh[i]); 
        } 

        println!("|----|");

      },
      Vertex::Empty => return,

    }

  }

}/*impl Vertex*/
