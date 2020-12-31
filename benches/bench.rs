extern crate bit_vec;
extern crate rand;

extern crate raphy;

use raphy::csr::CSR;
use criterion::{criterion_group, criterion_main, Criterion};
use bit_vec::BitVec;
use rand::Rng;

pub fn bfs(csr: &mut CSR){

  let mut bv = BitVec::from_elem(csr.get_v(),false);

  let mut rng = rand::thread_rng();
  let start: usize = rng.gen_range(0,csr.get_v()) as usize;
 
  csr.bfs_traversal(start,|v|bv.set(v,true));

} 
pub fn pagerank(csr: &mut CSR){

  let numv = csr.get_v();
  let nume = csr.get_e();
  let offs = csr.get_offsets();
  let neis = csr.get_neighbs();
   
  let iters = 100;
  let d: f64 = 0.85;
  let init_val: f64 = 1.0 / (numv as f64);
  
  /*Borrow vtxprop as a slice here to indicate that its size
    won't change, but as &mut because we'll be updating its entries*/
  let mut p_v = vec![init_val; numv];
  let mut p2_v = vec![init_val; nume];
      
  /*Double buffer swapping - start with p2_v because it has the initial values*/
  let p = &mut p_v;
  let p2 = &mut p2_v;

  for it in 0..iters{

    


    for i in 0..numv {    /*Iterate over vertices*/
  
      /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
      let (i_start,i_end) = (offs[i],
                             match i {
                               i if i == numv-1 => nume,
                               _ => offs[i+1] }
                            );

      let num_neighbs: f64 = i_end as f64 - i_start as f64;

      /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
      let mut n_upd: f64 = 0.0;
      for ei in i_start..i_end {
  
        let e = neis[ei];
        match e{ v1 => { 
              if it % 2 == 0{
                n_upd = n_upd + p2[v1] / num_neighbs; 
              }else{
                n_upd = n_upd + p[v1] / num_neighbs; 
              }
        } }
  
      }

      /*Update based on damping factor times identity vector + result*/
      if it % 2 == 0{
        p[i] = (1.0 - d) / (numv as f64) + d * n_upd; 
      }else{
        p2[i] = (1.0 - d) / (numv as f64) + d * n_upd; 
      }
  
    }

  }

  /*If last iteration filled the double buffer copy,
  put output in csr vertex prop array through p*/
  /*if iters % 2 != 0{
    for i in 0..len {
      csr.vtxprop[i] = p2_v[i];
    }
  }
  */

  /*for vi in 0..csr.v{
    println!("{} {}",vi,csr.vtxprop[vi]);
  }*/

} 


fn criterion_benchmark(c: &mut Criterion){

  const NUMV: usize = 100000;
  const MAXE: usize = 10;
  let mut csr = CSR::new(NUMV,CSR::random_el(NUMV,MAXE));
  c.bench_function("PageRank CSR |V|=100000 ~50 e / v:",|b| b.iter(|| pagerank(&mut csr)));
  
  c.bench_function("BFS CSR |V|=100000 ~10 e / v:",|b| b.iter(|| bfs(&mut csr)));

}

criterion_group!(benches,criterion_benchmark);
criterion_main!(benches);
