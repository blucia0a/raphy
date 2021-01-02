
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

fn main(){

  let (numv,el) = CSR::el_from_file("examples/in.el");
  let mut csr = CSR::new(numv,el);

  let mut vp = vec![0.0; numv];
  vp.clone_from_slice(csr.get_vtxprop());

  let vf = |_v0: usize, nei: &[usize]| {

    let mut sum: f64 = 0.0;
    for v1 in nei {

      sum = sum + vp[*v1];

    }
    sum

  };
  println!("Running Traversal");
  csr.par_scan(vf);

}
