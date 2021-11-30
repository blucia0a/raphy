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

extern crate bit_vec;
extern crate csv;
extern crate rand;

use bit_vec::BitVec;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

#[derive(Debug)]
pub struct CSR {
    v: usize,
    e: usize,
    vtxprop: Vec<f64>,
    offsets: Vec<usize>,
    neighbs: Vec<usize>,
}

impl CSR {
    pub fn get_vtxprop(&self) -> &[f64] {
        &self.vtxprop
    }

    pub fn get_mut_vtxprop(&mut self) -> &mut [f64] {
        &mut self.vtxprop
    }

    pub fn get_v(&self) -> usize {
        self.v
    }

    pub fn get_e(&self) -> usize {
        self.e
    }

    pub fn get_offsets(&self) -> &Vec<usize> {
        &self.offsets
    }

    pub fn get_neighbs(&self) -> &[usize] {
        &self.neighbs
    }

    /// Build a random edge list
    /// This method returns a tuple of the number of vertices seen and the edge list
    /// el.len() is the number of edges.  
    pub fn random_el(numv: usize, maxe: usize) -> Vec<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut el: Vec<(usize, usize)> = Vec::new();
        for i in 0..numv {
            /*edges per vertex*/
            let num_e: usize = rng.gen_range(0, maxe) as usize;
            for _ in 0..num_e {
                let edge = (i as usize, rng.gen_range(0, numv) as usize);

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
    pub fn el_from_file(path: &str) -> (usize, Vec<(usize, usize)>) {
        let mut el: Vec<(usize, usize)> = Vec::new();
        let mut maxv: usize = 0;

        let f = File::open(path);

        match f {
            Ok(file) => {
                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(file);
                for result in rdr.records() {
                    match result {
                        Ok(p) => {
                            let v0 = p.get(0).unwrap().parse::<usize>().unwrap();
                            let v1 = p.get(1).unwrap().parse::<usize>().unwrap();
                            if v0 > maxv {
                                maxv = v0
                            }
                            if v1 > maxv {
                                maxv = v1
                            }
                            el.push((v0, v1));
                        }
                        _ => {
                            eprintln!("Failed to parse file");
                        }
                    }
                }
            }
            _ => {
                eprintln!("Failed to open file {}", path);
            }
        }

        (maxv + 1, el)
    }

    /// Take an edge list in and produce a CSR out
    /// (u,v)
    pub fn new(numv: usize, ref el: Vec<(usize, usize)>) -> CSR {
        const NUMCHUNKS: usize = 16;
        let chunksz: usize = if numv > NUMCHUNKS {
            numv / NUMCHUNKS
        } else {
            1
        };

        /*TODO: Parameter*/
        let numbins = 16;

        let mut ncnt = Vec::new();
        for _ in 0..numv {
            ncnt.push(AtomicUsize::new(0));
        }

        /*Count up the number of neighbors that each vertex has */
        el.par_chunks(chunksz).for_each(|cnk| {
            /*Per-thread bin structure*/
            let mut bins = Vec::new();
            for _ in 0..numbins {
                bins.push(Vec::<&(usize, usize)>::new());
            }

            /*iterate over chunk, push edges to bins*/
            cnk.iter().for_each(|e| {
                bins[(e).0 % 16].push(e);
            });

            bins.iter().for_each(|b| {
                b.iter().for_each(|e| {
                    ncnt[(e).0].fetch_add(1, Ordering::SeqCst);
                });
            });
        });

        let mut work_offsets = Vec::new();
        work_offsets.push(AtomicUsize::new(0));

        let mut g = CSR {
            v: numv,
            e: el.len(),
            vtxprop: vec![0f64; numv],
            offsets: vec![0; numv],
            neighbs: vec![0; el.len()],
        };

        /* CSR Structure e.g.,
          |0,3,5,6,9|
          |v2,v3,v5|v1,v9|v2|v3,v7,v8|x|
        */
        /*vertex i's offset is vtx i-1's offset + i's neighbor count*/
        for i in 1..ncnt.len() {
            g.offsets[i] = g.offsets[i - 1] + ncnt[i - 1].load(Ordering::SeqCst);
            work_offsets.push(AtomicUsize::new(g.offsets[i]));
        }

        /*Temporary synchronized edge list array*/
        let mut nbs = Vec::new();
        for _ in 0..el.len() {
            nbs.push(AtomicUsize::new(0));
        }

        /*Populate the neighbor array based on the counts*/
        el.par_chunks(chunksz).for_each(|cnk| {
            cnk.iter().for_each(|edge| match *edge {
                (v0, v1) => {
                    let cur_ind = work_offsets[v0].fetch_add(1, Ordering::SeqCst);
                    nbs[cur_ind].store(v1, Ordering::Relaxed);
                }
            });
        });

        g.neighbs
            .par_chunks_mut(chunksz)
            .enumerate()
            .for_each(|(chunkbase, cnk)| {
                cnk.iter_mut().enumerate().for_each(|(i, e)| {
                    *e = nbs[chunkbase + i].load(Ordering::Relaxed);
                });
            });

        /*return the graph, g*/
        g
    }

    /// Get the range of offsets into the neighbs array that hold the neighbors
    /// of vertex v
    pub fn vtx_offset_range(&self, v: usize) -> (usize, usize) {
        (
            self.offsets[v],
            match v {
                v if v == self.v - 1 => self.e,
                _ => self.offsets[v + 1],
            },
        )
    }

    /// read_only_scan is a read only scan of all edges in the entire CSR
    /// that accepts a FnMut(usize,usize,u64) -> () to apply to each vertex
    pub fn read_only_scan(&self, mut f: impl FnMut(usize, usize) -> ()) {
        /*Iterate over the vertices in the offsets array*/
        let len = self.offsets.len();
        for i in 0..len {
            /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
            let (i_start, i_end) = self.vtx_offset_range(i);
            /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
            for ei in i_start..i_end {
                let e = self.neighbs[ei];
                match e {
                    v1 => {
                        f(i, v1);
                    }
                }
            }
        }
    }

    /// bfs_traversal starts from vertex start and does a breadth first search
    /// traversal on the vertices, applying f, the closure passed in, to each
    /// vertex
    pub fn bfs_traversal(&self, start: usize, mut f: impl FnMut(usize) -> ()) {
        let mut visited = BitVec::from_elem(self.v, false);
        let mut q = Vec::new();

        visited.set(start, true);
        q.push(start);

        while q.len() > 0 {
            let v = q.remove(0);

            f(v);

            let (st, en) = self.vtx_offset_range(v);

            for nei in st..en {
                /*Get the first element of the edge, which is the distal vertex*/
                let ne = self.neighbs[nei] as usize;

                match visited[ne] {
                    false => {
                        visited.set(ne, true);
                        q.push(ne as usize);
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn par_scan(&mut self, f: impl Fn(usize, &[usize]) -> f64 + std::marker::Sync) -> () {
        /*basically the number of threads to use*/
        const NUMCHUNKS: usize = 16;
        let chunksz: usize = if self.v > NUMCHUNKS {
            self.v / NUMCHUNKS
        } else {
            1
        };
        let scan_vtx_row = |(row_i, vtx_row): (usize, &mut [f64])| {
            let row_i_base: usize = row_i * chunksz;
            vtx_row
                .iter_mut()
                .enumerate()
                .for_each(|(ii, v): (usize, &mut f64)| {
                    let v0 = row_i_base + ii;
                    let (start, end) = self.vtx_offset_range(v0);
                    *v = f(v0, &self.neighbs[start..end]);
                });
        };

        let mut vtxprop = vec![0.0; self.get_v()];
        vtxprop
            .par_chunks_mut(chunksz)
            .enumerate()
            .for_each(scan_vtx_row);
        self.vtxprop.copy_from_slice(&vtxprop);
    }

    fn do_update(
        &self,
        v: &Vec<RwLock<f64>>,
        u: &mut Vec<RwLock<f64>>,
        f: impl Fn(f64, &[usize], &Vec<RwLock<f64>>) -> f64 + std::marker::Sync,
    ) {
        let numv = self.get_v();
        let nume = self.get_e();
        let offs = self.get_offsets();
        let neis = self.get_neighbs();

        (0..numv).into_par_iter().for_each(|i| {
            /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
            let (i_start, i_end) = (
                offs[i],
                match i {
                    i if i == numv - 1 => nume,
                    _ => offs[i + 1],
                },
            );

            /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
            let n_upd: f64 = f(*u[i].read().unwrap(), &neis[i_start..i_end], v);

            /*Update based on damping factor times identity vector + result*/
            let mut vprop = u[i].write().unwrap();
            *vprop = n_upd;
        });
    }

    pub fn update_traversal(
        &mut self,
        iters: usize,
        f: impl Fn(f64, &[usize], &Vec<RwLock<f64>>) -> f64 + std::marker::Sync,
    ) {
        let mut numv = 0;
        let mut v1: Vec<RwLock<f64>> = Vec::with_capacity(numv);
        {
            let csr_i = &*self;
            numv = csr_i.get_v();
            let vtxp = self.get_vtxprop();
            for v in 0..numv {
                v1.push(RwLock::<f64>::new(vtxp[v]));
            }
        }

        let mut v2: Vec<RwLock<f64>> = iter::repeat_with(|| RwLock::<f64>::new(0.0))
            .take(numv)
            .collect();

        for iter in 0..iters {

            let csr_i = &*self;
            let fr = &f;

            if iter % 2 == 0 {
                let v = &v1;
                let u = &mut v2;
                csr_i.do_update(v, u, fr);
            } else {
                let v = &v2;
                let u = &mut v1;
                csr_i.do_update(v, u, fr);
            }
        }

        {
            let vtxp = self.get_mut_vtxprop();
            //for i in 0..numv {
            vtxp.iter_mut().enumerate().for_each(|(i,v)|{
                if iters % 2 == 0 {
                    *v = *v1[i].read().unwrap();
                } else {
                    *v = *v2[i].read().unwrap();
                }
            });
        }
    }

} /*impl CSR*/
