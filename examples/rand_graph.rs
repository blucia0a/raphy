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

fn main(){

  let mut rng = rand::thread_rng();
  let mut el = Vec::new();
 
  const NUMV: usize = 100;

  /*edges per vertex*/
  const NUME: usize = 10;
  
  const MAX_WEIGHT: usize = 1000;

  for i in 0..NUMV { 

    for _ in 0..NUME {
  
      let edge = (i as usize, rng.gen_range(0,NUMV) as usize, rng.gen_range(0,MAX_WEIGHT) as u64);

      println!("{} {} {}",edge.0,edge.1,edge.2);

      el.push(edge);

    }

  }
  
  let gg = Graph::new(NUMV,el);
  gg.print();

}
