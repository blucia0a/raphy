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

use byte_slice_cast::*;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub struct FastCSR {
    v: usize,
    e: usize,
    obase: usize,
    nbase: usize,
    raw: Box<Mmap>,
}

impl FastCSR {
    pub fn getv(&self) -> usize {
        self.v
    }

    pub fn gete(&self) -> usize {
        self.e
    }

    pub fn new(s: String) -> FastCSR {
        let path = PathBuf::from(s);
        let file = OpenOptions::new().read(true).open(&path).unwrap();

        let mmap = Box::new(unsafe { Mmap::map(&file).unwrap() });

        assert!(mmap.len() >= 8);
        let csr = mmap[..]
                  .as_slice_of::<usize>()
                  .unwrap();

        let v = csr[0];
        let e = csr[1];

        println!("{} edges total", e);
        FastCSR {
            v: v,
            e: e,
            obase: 16,
            nbase: 16 + v * 8,
            raw: mmap,
        }
    }

    pub fn offset(&self, i: usize) -> usize {
        let offsets = &self.raw[self.obase..self.nbase]
            .as_slice_of::<usize>()
            .unwrap();

        offsets[i]
    }

    pub fn neighbors(&self, i: usize) -> &[usize] {
        let (n0, nn) = self.vtx_offset_range(i);
        let edges = &self.raw[self.nbase..].as_slice_of::<usize>().unwrap();
        &edges[n0..nn]
    }

    fn vtx_offset_range(&self, v: usize) -> (usize, usize) {
        (
            self.offset(v),
            match v {
                v if v == self.v - 1 => self.e,
                _ => self.offset(v + 1),
            },
        )
    }
    pub fn neighbor_scan_prop(&self, f: impl Fn(usize, &[usize]) -> f64 + std::marker::Sync, prop: &mut [f64]) {
        prop.par_iter_mut().enumerate().for_each(|(v,p)| {
            let (n0, nn) = self.vtx_offset_range(v);
            let edges = &self.raw[self.nbase..].as_slice_of::<usize>().unwrap();
            let res = f(v, &edges[n0..nn]);
            *p = res;
        });
    }

    pub fn neighbor_scan(&self, f: impl Fn(usize, &[usize]) -> () + std::marker::Sync) {
        (0..self.v).into_par_iter().for_each(|v| {
            let (n0, nn) = self.vtx_offset_range(v);
            let edges = &self.raw[self.nbase..].as_slice_of::<usize>().unwrap();
            f(v, &edges[n0..nn]);
        });
    }

    pub fn read_only_scan(&self, f: impl Fn(usize, usize) -> () + std::marker::Sync) {
        (0..self.v).into_par_iter().for_each(|v| {
            let (n0, nn) = self.vtx_offset_range(v);
            let edges = self.raw[self.nbase..].as_slice_of::<usize>().unwrap();
            edges[n0..nn].into_iter().for_each(|n| {
                f(v, *n);
            });
        });
    }
} /*impl FastCSR*/
