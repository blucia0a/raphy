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

extern crate rand;
use rand::Rng;

extern crate raphy;
use raphy::graph::Graph;
use raphy::vertex::Vertex;

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

  println!("Running iterator-based print traversal over vertices");
  for v in gg.iter() {
    v.print();
  }
  
  println!("Iterating through vertices, incrementing value, preserving rest");
  let n = gg.num_vtxs();
  for i in 0..n {

    let v = gg.get_vtx(i);
    let mut old_id = 0; 
    let mut old_val = 0; 
    let mut old_neigh = vec![];
    match v{
      Vertex::V{ ref id, ref val, ref neigh} => { 
        old_id = *id; 
        old_val = *val; 
        for ne in neigh{
          let new_neigh: u64 = **ne;
          old_neigh.push(Box::new(new_neigh));
        }
      },
      Vertex::Empty => (),
    }
    gg.set_vtx(i, Vertex::V{ id: old_id, val: old_val + 1, neigh: old_neigh });
    
    gg.get_vtx(i).print();
  }

  println!("Printing whole graph (should match above)");
  gg.print();

}
