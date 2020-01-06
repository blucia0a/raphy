use std::cmp::Ordering;

//extern crate rand;
//use rand::Rng;

/*Vertex requires printability and ordering*/
trait VtxTrait: Ord + std::fmt::Debug + std::fmt::Display {}
impl<T> VtxTrait for T where T: Ord + std::fmt::Debug + std::fmt::Display {}

#[derive(Debug)]
enum Graph<'a, T: VtxTrait> {
  G{
    vtxs: Vec< Box< &'a Vertex<T> > >,
    next_vtx: usize,
  },
  Empty, 
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

impl<'a, T: VtxTrait> Graph<'a, T> {
  fn new() -> Graph<'a, T>{
    Graph::G{ vtxs: vec![], next_vtx: 0}
  } 

  fn create_vtx(&mut self, v: T) -> Vertex<T>{
    match self{
      Graph::G{ ref mut next_vtx, .. } => { 
        let next_id = *next_vtx;
        *next_vtx = *next_vtx + 1;
        Vertex::new(next_id as u64, v)
      },
      Graph::Empty => { Vertex::new(0,v) }
    }
  }

  fn add_vtx(&mut self, v: &'a Vertex<T>){

    let mut ins_id: usize = 0;
    match v{
      Vertex::V{ ref id, ref val, ref neigh } => ins_id = *id as usize,
      Vertex::Empty => return,
    };

    match self{
      Graph::G{ ref mut vtxs, .. } => { vtxs.insert(ins_id, Box::new(v)); },
      Graph::Empty => return,
    };
  }

  fn print(&self){
    match self{
    Graph::G{ ref vtxs, .. } => { for i in 0..vtxs.len() { vtxs[i].print(); } },
    _ => return
    }
  }
}

impl<T: VtxTrait> Vertex<T> {

  fn new(id: u64, nv: T) -> Vertex<T> {

    Vertex::V{ id: id, val: nv, neigh: vec![] }

  }

  //fn add_neigh(&mut self, id: u64) -> Result{//TODO
  fn add_id(&mut self, id: u64){

    match self{

      &mut Vertex::V{ ref id, ref val, ref mut neigh}  => {
        neigh.push( Box::new(*id) );
      },

      &mut Vertex::Empty => {
        return;
      }

    }

  }
  
  fn add_vtx(&mut self, nv: &Vertex<T>){

    match self{

      &mut Vertex::V{ ref id, ref val, ref mut neigh}  => {

        let mut new_id: u64 = 0; 
        match nv{
          Vertex::V{ ref id, ref val, ref neigh } => new_id = *id,
          Vertex::Empty => { return; }
        }
        neigh.push(Box::new(new_id))

      },
      &mut Vertex::Empty => { return; }

    }

  }

  fn find(&self, fv: T) -> bool {
    true
  }

  fn print(&self){

    match self{

      Vertex::V{ ref id, ref val, ref neigh } => { 

        println!("{} => {}\n|----|",id,val); 

        for i in 0..neigh.len() { 
          println!("{}",neigh[i]); 
        } 

        println!("|----|");

      },

      _ => return

    }

  }
  
  fn traverse(&self, func: fn(v: &T)){
  }
  
}

fn prt<T: VtxTrait>(v: &T){

}



fn main(){

  let mut g: Graph<u64> = Graph::new();
  let mut t = g.create_vtx(9264);
  let mut u = g.create_vtx(8111);
  let mut v = g.create_vtx(7777);
  let mut z = g.create_vtx(6666);

  t.add_vtx(&u);
  t.add_vtx(&v);
  g.add_vtx(&t);

  u.add_vtx(&v);
  u.add_vtx(&z);
  g.add_vtx(&u);

  v.add_vtx(&z);
  v.add_vtx(&t);
  g.add_vtx(&v);

  z.add_vtx(&t);
  z.add_vtx(&u);
  g.add_vtx(&z);

  g.print();

}
