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

fn main() {
    const NUMITERS: usize = 10;
    let (numv, el) = CSR::el_from_file("examples/in.el");
    let mut csr = CSR::new(numv, el);

    csr.get_mut_vtxprop()
        .iter_mut()
        .for_each(|vp: &mut f64| *vp = 1.0 / (numv as f64));

    for _ in 0..NUMITERS {
        let mut vp = vec![0.0; numv];
        vp.clone_from_slice(csr.get_vtxprop());

        /*The closure returns the value that should be stored in csr.vtxprop
        for v0*/
        let vf = |_v0: usize, nei: &[usize]| {
            const D: f64 = 0.85;
            let mut n_upd: f64 = 0.0;

            nei.iter()
                .for_each(|v1| n_upd = n_upd + vp[*v1] / (nei.len() as f64));

            (1.0 - D) / (numv as f64) + D * n_upd
        };

        println!("Running Traversal");
        csr.par_scan(vf);
    }

    csr.get_vtxprop()
        .iter()
        .enumerate()
        .for_each(|(i, v)| println!("{} {}", i, v));
}
