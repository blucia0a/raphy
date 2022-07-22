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

use memmap2::Mmap;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::convert::TryInto;
use rayon::prelude::*;

pub struct FastCSR {
    v: usize,
    e: usize,
    raw: Box<Mmap>
}

impl FastCSR {

  fn as_u64_le(array: &[u8; 8]) -> u64 {
        ((array[0] as u64) <<  0) +
        ((array[1] as u64) <<  8) +
        ((array[2] as u64) << 16) +
        ((array[3] as u64) << 24) +
        ((array[4] as u64) << 32) +
        ((array[5] as u64) << 40) +
        ((array[6] as u64) << 48) +
        ((array[7] as u64) << 56)
  }

  pub fn getv(&self) -> usize {
    self.v
  }
  
  pub fn gete(&self) -> usize {
    self.e
  }

  pub fn new(s: String) -> FastCSR {

    let path = PathBuf::from(s);
    let file = OpenOptions::new()
                             .read(true)
                             .open(&path).unwrap();

    let mmap = Box::new(unsafe {  Mmap::map(&file).unwrap() });

    assert!(mmap.len() >= 8);

    let offsets_len: usize = FastCSR::as_u64_le(&mmap[0..8].try_into().unwrap()) as usize;
    let neighbs_len: usize = FastCSR::as_u64_le(&mmap[8..16].try_into().unwrap()) as usize;
    println!("{} {}",offsets_len,neighbs_len);
    
    FastCSR {

      v: offsets_len,
      e: neighbs_len,
      raw: mmap 

    }

  }  

  pub fn offset(&self, i: usize) -> u64{
    let j = i * 8; 
    FastCSR::as_u64_le(self.raw[(16+j)..(16+j)+8].try_into().unwrap())
  }
    
  fn getchunksize(i: usize) -> usize{

    const NUMCHUNKS: usize = 16;
    if i > NUMCHUNKS {
      i / NUMCHUNKS
    } else {
      1
    }

  }
    
  fn vtx_offset_range(&self, v: usize) -> (usize, usize) {

        (
            self.offset(v) as usize,
            match v {
                v if v == self.v - 1 => self.e as usize,
                _ => self.offset(v + 1) as usize,
            },
        )

  }
  
  pub fn read_only_scan(&self, f: impl Fn(usize,usize) -> () + std::marker::Sync){

    let chunksz = FastCSR::getchunksize(self.v);
    (0..self.v).into_par_iter()
                  .for_each(|v| {

      let (n0,nn) = self.vtx_offset_range(v);
      (n0..nn).into_par_iter()
              .for_each(|n|{

        //nth neighbor is at offset n*8 from the base of the neighbors array
        //which is at 2*8 + v*8 + n*8
        let ni = 2*8 + self.v*8  + n*8;
       
        let nv = FastCSR::as_u64_le(self.raw[ni..ni+8].try_into().unwrap()) as usize;
        f(v,nv);

      });

    });

  }

  pub fn print(&self){

    let chunksz = FastCSR::getchunksize(self.v);
    (0..self.v).into_par_iter()
                  .for_each(|v| {

      let (n0,nn) = self.vtx_offset_range(v);
      (n0..nn).into_par_iter()
              .for_each(|n|{

        //nth neighbor is at offset n*8 from the base of the neighbors array
        //which is at 2*8 + v*8 + n*8
        let ni = 2*8 + self.v*8  + n*8;
       
        let nv = FastCSR::as_u64_le(self.raw[ni..ni+8].try_into().unwrap());
        println!("{:#x} --> {:#x}",v,nv);

      });

    });

  }
  

} /*impl FastCSR*/
