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
//use crate::VtxTrait;
//use crate::vertex::Vertex;

/*TODO: Need to add in a vertex property array 
        separate from the graph structure arrays*/
#[derive(Debug)]
pub struct Graph {
  //vprop: Vec< Box<T> >,
  v: usize,
  e: usize,
  offsets: Vec< usize >,
  neighbs: Vec< (usize,u64) >
}


impl Graph {

  /*Take an edge list in and produce a Graph out*/
  /*(u,v) -> weight*/
  pub fn new(numv: usize, ref el: Vec<(usize,usize,u64)>) -> Graph{

    let mut ncnt = Vec::with_capacity(numv);
    for i in 0..numv {

      ncnt[i] = 0;

    }
    
    /*Count up the number of neighbors that each vertex has */ 
    for edge in el {

      match *edge {

        (v0,_,_) => { ncnt[v0] = ncnt[v0] + 1; }

      }

    }  
    
    let mut g = Graph{ 

      v: numv,
      e: el.len(),
      offsets: Vec::with_capacity(numv), 
      neighbs: Vec::with_capacity(el.len()) 

    };
 
    /*vertex i's offset is vtx i-1's offset + i's neighbor count*/
    g.offsets[0] = 0; 
    for i in 1..ncnt.len() {

      g.offsets[i] = g.offsets[i-1] + ncnt[i-1];

    }
 
    /*Populate the neighbor array based on the counts*/ 
    for edge in el {

      match *edge {

        /*use offsets array to fill edges into the neighbs array*/
        (v0,v1,weight) => {  }

      }

    }  

    /*return the graph, g*/
    g

  } 

}/*impl Graph*/
