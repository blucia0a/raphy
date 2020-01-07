extern crate rand;
use rand::Rng;

extern crate raphy;
use raphy::graph::Graph;

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
