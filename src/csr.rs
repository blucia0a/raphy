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
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Debug)]
pub struct CSR{
  v: usize,
  e: usize,
  pub offsets: Vec< usize >,
  pub neighbs: Vec< usize >
}

impl CSR{
  

  pub fn get_v(&self) -> usize{
    self.v 
  }
  
  pub fn get_e(&self) -> usize{
    self.e 
  }

  pub fn get_offsets(&self) -> &Vec<usize>{
    &self.offsets 
  }

  pub fn get_neighbs(&self) -> &Vec<usize>{
    &self.neighbs
  }

  /// Build a random edge list
  /// This method returns a tuple of the number of vertices seen and the edge list
  /// el.len() is the number of edges.  
  pub fn random_el(numv: usize, maxe: usize) -> Vec< (usize,usize) >{

    let mut rng = rand::thread_rng();
    let mut el: Vec::<(usize,usize)> = Vec::new();
    for i in 0..numv{ 

      /*edges per vertex*/
      let num_e: usize = rng.gen_range(0,maxe) as usize;
      for _ in 0..num_e{
  
        let edge = (i as usize, rng.gen_range(0,numv) as usize);

        el.push(edge);

      }

    }

    el

  }


  /// Build an edge list from a file containing text describing one.
  /// The file format is line oriented and human readable:
  /// v0,v1
  /// v0,v2
  /// v0,v3
  /// v0,v3
  /// v1,v2
  /// v1,v2
  /// v2,v3
  /// v3,v1
  /// ...
  ///
  /// This method returns a tuple of the number of vertices seen and the edge list
  /// el.len() is the number of edges.  
  pub fn el_from_file(path: &str) -> (usize, Vec< (usize,usize) >){

    let mut el: Vec::<(usize,usize)> = Vec::new();
    let mut maxv: usize = 0;

    let f= File::open(path);

    match f {
      Ok(file) => {
        let reader = BufReader::new(file);
        for line in reader.lines() {
          let ln = line.unwrap();
          let linesplit = ln.split(",");
          let tup_v: Vec<&str> = linesplit.collect(); 

          let v0 = tup_v[0].parse::<usize>().unwrap();
          let v1 = tup_v[1].parse::<usize>().unwrap();

          if v0 > maxv { maxv = v0 }
          if v1 > maxv { maxv = v1 }
           
          el.push( (v0,v1) );
        }
        
      },

      _ => {
        println!("Failed to open file {}",path);
      }

    }
    (maxv+1,el)

  }


  /// Take an edge list in and produce a CSR out
  /// (u,v)
  pub fn new(numv: usize, ref el: Vec<(usize,usize)>) -> CSR{

    let mut ncnt = Vec::new();
    for _ in 0..numv {

      ncnt.push(0);

    }
    
    /*Count up the number of neighbors that each vertex has */ 
    for edge in el {

      match *edge {

        (v0,_) => { ncnt[v0] = ncnt[v0] + 1; }

      }

    }  
    
    let mut g = CSR{ 

      v: numv,
      e: el.len(),
      offsets: Vec::new(),
      neighbs: Vec::new(),

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
      g.neighbs.push(0);
    }

    for i in 1..ncnt.len() {

      g.offsets[i] = g.offsets[i-1] + ncnt[i-1];
      work_offsets[i] = g.offsets[i];
    }
 
    /*Populate the neighbor array based on the counts*/ 
    for edge in el {

      match *edge {

        /*use offsets array to fill edges into the neighbs array*/
        (v0,v1) => {  

	  /*The vertex of the index increments with each adjacency until
           * hitting the base index of the next vertex*/
          let cur_ind = work_offsets[v0];
          work_offsets[v0] = work_offsets[v0] + 1; 

          /*Install the vertex into the CSR*/
          g.neighbs[cur_ind] = v1; 

        }

      }

    }  

    /*return the graph, g*/
    g

  } 

  /// Get the range of offsets into the neighbs array that hold the neighbors
  /// of vertex v
  pub fn vtx_offset_range(&self, v: usize) -> (usize,usize){
    (self.offsets[v],
     match v {
      v if v == self.v-1 => self.e,
      _ => self.offsets[v+1]
     })
  }

  /// read_only_scan is a read only scan of all edges in the entire CSR
  /// that accepts a FnMut(usize,usize,u64) -> () to apply to each vertex
  pub fn read_only_scan(&self, mut f: impl FnMut(usize,usize) -> ()){

    /*Iterate over the vertices in the offsets array*/
    let len = self.offsets.len();
    for i in 0..len {

      /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
      let (i_start,i_end) = self.vtx_offset_range(i);

      /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
      for ei in i_start..i_end {

        let e = self.neighbs[ei];
        match e{

          v1 => {

            f(i,v1);   

          }

        }

      }

    }

  }

  /// bfs_traversal starts from vertex start and does a breadth first search 
  /// traversal on the vertices, applying f, the closure passed in, to each vertex
  pub fn bfs_traversal(&self, start: usize, mut f: impl FnMut(usize) -> ()){
   
    let mut visited = vec![false; self.v]; 
    let mut q = Vec::new();
    visited[start] = true;
    q.push(start);

    while q.len() > 0 {

      let v = q.remove(0);

      f(v);

      let (st,en) = self.vtx_offset_range(v);

      for nei in st..en {

        /*Get the first element of the edge, which is the distal vertex*/
        let ne = self.neighbs[nei] as usize;
      
        match visited[ne]{
          false => {
  
            visited[ne] = true;
            q.push(ne as usize);
            
          }
          _ => ()
  
        }

      }

    }

  }



}/*impl CSR*/
