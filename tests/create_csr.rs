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
extern crate raphy;
use raphy::csr::CSR;
use std::fs::{self,File};
use std::io::{Write,BufWriter};
use crate::rand::Rng;

#[test]
fn test_csr_build(){

  const NUMV: usize = 1000;
  const MAXE: usize = 10;
  let el = CSR::random_el(NUMV,MAXE);

  {

    let file = File::create("tmp.el").unwrap();
    let mut writer = BufWriter::new(&file);

    for (v0,v1) in &el {
      let _ = write!(&mut writer,"{},{}\n",v0,v1);
    } 
 
  }
  
  let (numv2,el2) = CSR::el_from_file("tmp.el");
  assert_eq!(el.len(),el2.len());

  let csr = CSR::new(NUMV,el);
  let csr2 = CSR::new(numv2,el2);

  let _ = fs::remove_file("tmp.el");
  /*
    Run a BFS on the csr built from the in-memory EL
    and from the EL written to and loaded from the file
    and be sure that they produce the same result
  */
  let mut rng = rand::thread_rng();
  let start_v: usize = rng.gen_range(0,NUMV) as usize;
  let mut a = Vec::new();
  let mut b = Vec::new();
  csr.bfs_traversal(start_v,|v| a.push(v));
  csr2.bfs_traversal(start_v,|v| b.push(v));

  /*quick element-wise vector comparison hack taken from: 
    https://stackoverflow.com/questions/29504514/
    whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
  */
  let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
  assert!(matching == a.len() && matching == b.len());
 
}
