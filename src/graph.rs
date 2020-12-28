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

#[derive(Debug)]
pub struct CSR{
  //vprop: Vec< Box<T> >,
  v: usize,
  e: usize,
  offsets: Vec< usize >,
  neighbs: Vec< (usize,u64) >
}

impl CSR{

  /*Take an edge list in and produce a Graph out*/
  /*(u,v) -> weight*/
  pub fn new(numv: usize, ref el: Vec<(usize,usize,u64)>) -> CSR{

    let mut ncnt = Vec::new();
    for _ in 0..numv {

      ncnt.push(0);

    }
    
    /*Count up the number of neighbors that each vertex has */ 
    for edge in el {

      match *edge {

        (v0,_,_) => { ncnt[v0] = ncnt[v0] + 1; }

      }

    }  
    
    let mut g = CSR{ 

      v: numv,
      e: el.len(),
      offsets: Vec::new(),
      neighbs: Vec::new()
      /*offsets: Vec::with_capacity(numv), 
      neighbs: Vec::with_capacity(el.len()) */

    };


    /*|0,3,5,6,9|
      |v2,v3,v5|v1,v9|v2|v3,v7,v8|x|
    */
    /*vertex i's offset is vtx i-1's offset + i's neighbor count*/
    let mut work_offsets = Vec::new();
    for _ in 0..numv {
      work_offsets.push(0);
      g.offsets.push(0);
    }

    for _ in 0..el.len(){
      g.neighbs.push((0,0,));
    }

    for i in 1..ncnt.len() {

      g.offsets[i] = g.offsets[i-1] + ncnt[i-1];
      work_offsets[i] = g.offsets[i];
    }
 
    /*Populate the neighbor array based on the counts*/ 
    for edge in el {

      match *edge {

        /*use offsets array to fill edges into the neighbs array*/
        (v0,v1,weight) => {  

	  /*The vertex of the index increments with each adjacency until
           * hitting the base index of the next vertex*/
          let cur_ind = work_offsets[v0];
          work_offsets[v0] = work_offsets[v0] + 1; 

          /*Install the vertex into the CSR*/
          g.neighbs[cur_ind] = (v1,weight); 

        }

      }

    }  

    /*return the graph, g*/
    g

  } 


  pub fn read_only_traversal(&self,f: impl Fn(usize,usize,u64) -> ()){

    /*Iterate over the vertices in the offsets array*/
    let len = self.offsets.len();
    for i in 0..len {

      /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
      let i_start = self.offsets[i];
      let i_end = match i {
        i if i == len-1 => self.neighbs.len(),
        _ => self.offsets[i+1]
      };

      /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
      for ei in i_start..i_end {

        let e = self.neighbs[ei];
        match e{

          (v1,weight) => {

            f(i,v1,weight);   

          }

        }

      }

    }

  }



}/*impl CSR*/
