//use std::cmp::Ordering;

extern crate rand;
use rand::Rng;


/*Vertex requires printability and ordering*/
trait VtxTrait: Ord + std::fmt::Debug + std::fmt::Display {}
impl<T> VtxTrait for T where T: Ord + std::fmt::Debug + std::fmt::Display {}

#[derive(Debug)]
enum Graph<T: VtxTrait> {
  G{
    vtxs: Vec< Box< Vertex<T> > >,
    next_vtx: usize,
  },
}

#[derive(Debug)]
enum Vertex<T: VtxTrait> {
  V{
    id: u64,
    val: T,
    neigh: Vec<Box<u64>>,
  },
  Empty,
}

impl<T: VtxTrait> Graph< T> {
  fn new() -> Graph<T>{
    Graph::G{ vtxs: vec![], next_vtx: 0}
  } 

  fn init_vtx(&mut self, ind: u64, v: T){
    match self{
      Graph::G{ ref mut vtxs, ..} => {
        vtxs[ind as usize].init(ind,v);
      }
    }
  }

  fn create_vtx(&mut self){

    match self{

      Graph::G{ ref mut next_vtx, ref mut vtxs, .. } => { 

        let next_id = *next_vtx;

        *next_vtx = *next_vtx + 1;

        vtxs.insert(next_id, Box::new(Vertex::Empty) );

      },

    }

  }

  fn add_vtx(&mut self, v: Vertex<T>){

    let ins_id: usize;
    match v{
      Vertex::V{ ref id, .. } => ins_id = *id as usize,
      Vertex::Empty => return,
    };

    match self{
      Graph::G{ ref mut vtxs, .. } => { vtxs.insert(ins_id, Box::new(v)); },
    };
  }

  fn print(&self){
    match self{
    Graph::G{ ref vtxs, .. } => { for i in 0..vtxs.len() { vtxs[i].print(); } },
    }
  }
}

impl<T: VtxTrait> Vertex<T> {

  fn new(id: u64, nv: T) -> Vertex<T> {

    Vertex::Empty

  }

  fn init(&mut self, new_id: u64, nv: T){
    match self{
      &mut Vertex::V{ .. } => {},
      &mut Vertex::Empty => {
        *self = Vertex::V{ id: new_id, val: nv, neigh: vec![] };
      }
    }
  }

  fn add_vtx(&mut self, nv: &Vertex<T>){

    match self{

      &mut Vertex::V{ ref mut neigh, .. }  => {

        let new_id: u64; 
        match nv{
          Vertex::V{ ref id, .. } => new_id = *id,
          Vertex::Empty => return,
        }
        neigh.push(Box::new(new_id))

      },
      Vertex::Empty => return,
    }

  }

/*
  fn find(&self, fv: T) -> bool {
    true
  }
*/

  fn print(&self){

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
/*  
  fn traverse(&self, func: fn(v: &T)){
  }
*/
}
/*
fn prt<T: VtxTrait>(v: &T){

}
*/


fn main(){

  let mut rng = rand::thread_rng();

  let mut gg: Graph<u64> = Graph::new();
  for i in 0..1000 {
    gg.create_vtx( );
    gg.init_vtx( i, rng.gen_range(0,2000000) as u64 );
    gg.add_neigh( i, 
  }
  gg.print();

}
