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

fn bfs(gg: &Graph<u64>, start: usize){

  /*Initialize visited list and worklist*/
  let mut visited: Vec<bool> = Vec::new();
  for _ in 0..gg.num_vtxs() { visited.push(false) }
  let mut q: Vec<usize> = Vec::new();

  visited[start] = true;
  q.push(start);

  /*Traverse while the worklist is not empty*/
  while q.len() > 0 {

    let vi = q.remove(0);
    let v = gg.get_vtx(vi);
    match v{

      Vertex::V{ id: _, val: _, ref neigh} => { 

        for ne in neigh{

          let ni: usize = **ne as usize;

          if visited[ni] == false {

            visited[ni as usize] = true;
            q.push(ni as usize);

          }

        }

      },

      Vertex::Empty => (),

    }
    //visit(&gg,v,visited.as_mut_slice(),&mut q); 

  }

  let mut s = String::new();
  let mut cnt: u64 = 0;
  for v in 0..visited.len() {

    match visited[v] {


      true => {
        let ff = format!("{},",v);
        let ffs = &ff[..];
        s.push_str(ffs); 
        cnt = cnt + 1;
      },

      false => ()

    }

  } 

  println!("Visited {} vertices",cnt);
 // println!("{}: [{}]",start,s);

}

fn main(){

  const NUMV: u64 = 500;
  const NUMN: u64 = 5;
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
  
  println!("Graph Constructed.");

  //gg.print();

  println!("Traversing...");

  /*Choose random start vertex id*/
  //let start = rng.gen_range(0,NUMV) as usize;
  for start in 0..NUMV {

    bfs(&gg,start as usize);

  }

}
