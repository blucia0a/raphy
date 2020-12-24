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

fn visit(g: &Graph<u64>, v: &Vertex<u64>, visited: &mut [bool]){

    match v{

      Vertex::V{ ref id, val: _, ref neigh} => { 

        //println!("V: {}",*id as u64);

        visited[*id as usize] = true;

        for ne in neigh{

          let ni: usize = **ne as usize;

          if visited[ni] == false {

            //println!("\t{}",ni);

            let n = g.get_vtx(ni);

            visit(g,n,visited);

          }

        }

      },

      Vertex::Empty => (),

    }

}

fn main(){

  const NUMV: u64 = 10000;
  const NUMN: u64 = 2000;
  const VALRNG: u64 = 2000000;

  let mut rng = rand::thread_rng();

  let mut gg: Graph<u64> = Graph::new();

  for i in 0..NUMV {

    gg.add_vtx( i, rng.gen_range(0,VALRNG) as u64 );

    let jb = rng.gen_range(0,NUMN);
    for _ in 0..jb {
      
      gg.add_edge( i, rng.gen_range(0,NUMV) as u64 ); 

    }

  }
  
  println!("Graph Constructed. Traversing...");

  let n = gg.num_vtxs();

  let mut visited: Vec<bool> = Vec::new();
  for _ in 0..n { visited.push(false) }

  for i in 0..n {

    let v = gg.get_vtx(i);

    visit(&gg,v,visited.as_mut_slice()); 

  }

  let mut cnt: u64 = 0;
  for v in visited.iter() {
    match v {
      true => cnt = cnt + 1,
      false => ()
    }
  } 
  println!("Visited {} vertices",cnt);

}
