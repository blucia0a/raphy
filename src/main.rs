extern crate rand;
use rand::Rng;

/*Vertex requires printability and ordering*/
trait VtxTrait: Ord + std::fmt::Debug + std::fmt::Display {}
impl<T> VtxTrait for T where T: Ord + std::fmt::Debug + std::fmt::Display {}

#[derive(Debug)]
struct Graph<T: VtxTrait> {
  vtxs: Vec< Box< Vertex<T> > >,
  next_vtx: u64,
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
    Graph{ vtxs: vec![], next_vtx: 0}
  } 

  fn add_vtx(&mut self, ind: u64, v: T){
    self.create_vtx( );
    self.init_vtx(ind, v); 
  }
  
  fn add_edge(&mut self, ind: u64, nei: u64){
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

  fn print(&self){
    for i in 0..self.vtxs.len() { self.vtxs[i].print(); }
  }

}/*Graph*/

impl<T: VtxTrait> Vertex<T> {

  fn init(&mut self, new_id: u64, nv: T){
    match self{
      &mut Vertex::V{ .. } => {},
      &mut Vertex::Empty => {
        *self = Vertex::V{ id: new_id, val: nv, neigh: vec![] };
      }
    }
  }

  fn add_neigh(&mut self, n_id: u64){

    match self{

      &mut Vertex::V{ ref mut neigh, .. }  => {
        neigh.push(Box::new(n_id));
      },
      Vertex::Empty => return,

    }

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
      Vertex::Empty => return,

    }

  }

}

fn main(){

  let mut rng = rand::thread_rng();

  let mut gg: Graph<u64> = Graph::new();

  for i in 0..10 {

    gg.add_vtx( i, rng.gen_range(0,2000000) as u64 );

    let jb = rng.gen_range(0,5);
    for _ in 0..jb {
      
      gg.add_edge( i, rng.gen_range(0,10) as u64 ); 

    }

  }
  gg.print();

}
