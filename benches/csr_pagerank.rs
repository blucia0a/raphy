extern crate rand;
extern crate raphy;
use raphy::csr::CSR;
use criterion::{criterion_group, criterion_main, Criterion};

  
pub fn pagerank(csr: &mut CSR){

  let len = csr.v;
  let iters = 100;
  let d: f64 = 0.85;
  let init_val: f64 = 1.0 / (len as f64);

  /*Borrow vtxprop as a slice here to indicate that its size
    won't change, but as &mut because we'll be updating its entries*/
  let mut p2_v = vec![init_val; len];

  /*Double buffer swapping - start with p2_v because it has the initial values*/
  let p = &mut csr.vtxprop;
  let p2 = &mut p2_v;
  for it in 0..iters{

    /*Iterate over vertices*/
    for i in 0..len {
  
      /*A vertex i's offsets in neighbs array are offsets[i] to offsets[i+1]*/
      let (i_start,i_end) = (csr.offsets[i],
                             match i {
                               i if i == csr.v-1 => csr.e,
                               _ => csr.offsets[i+1] }
                            );

      let num_neighbs: f64 = i_end as f64 - i_start as f64;

      /*Traverse vertex i's neighbs and call provided f(...) on the edge*/
      let mut n_upd: f64 = 0.0;
      for ei in i_start..i_end {
  
        let e = csr.neighbs[ei];
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
        p[i] = (1.0 - d) / (csr.v as f64) + d * n_upd; 
      }else{
        p2[i] = (1.0 - d) / (csr.v as f64) + d * n_upd; 
      }
  
    }


  }

  /*If last iteration filled the double buffer copy,
  put output in csr vertex prop array through p*/
  if iters % 2 != 0{
    for i in 0..len {
      csr.vtxprop[i] = p2_v[i];
    }
  }

  /*for vi in 0..csr.v{
    println!("{} {}",vi,csr.vtxprop[vi]);
  }*/

} 


fn criterion_benchmark(c: &mut Criterion){

  const NUMV: usize = 100000;
  const MAXE: usize = 10;
  let mut csr = CSR::new(NUMV,CSR::random_el(NUMV,MAXE));
  c.bench_function("PageRank CSR |V|=100000 ~50 e / v:",|b| b.iter(|| pagerank(&mut csr)));

}

criterion_group!(benches,criterion_benchmark);
criterion_main!(benches);
