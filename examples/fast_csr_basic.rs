extern crate rand;

extern crate raphy;
use raphy::fast_csr::FastCSR;
use std::sync::RwLock;

fn main() {

    let fcsr = FastCSR::new(String::from("./big.csr")); 

    let mut vp1 = Vec::new(); 
    for _ in 0..fcsr.getv() {
      vp1.push(RwLock::new(0.0));
    }

    for _ in 0..10 {

      let mut vp2 = Vec::new(); 
      for _ in 0..fcsr.getv() {
        vp2.push(RwLock::new(0.0));
      }
  
      let vf = |v: usize , nei: &[usize]|{
  
        const D: f64 = 0.85;
        let mut n_upd: f64 = 0.0;
  
        nei.iter()
           .for_each(|v1| n_upd = n_upd + *vp1[*v1].read().unwrap() / (nei.len() as f64));
  
        {
          let mut prop = vp2[v].write().unwrap();
          *prop = (1.0 - D) / (fcsr.getv() as f64) + D * n_upd;
        }
  
      };
  
      fcsr.neighbor_scan(vf);
      for v in 0..(vp1.len()-1) {
        *vp1[v].write().unwrap() = *vp2[v].read().unwrap();
      } 

   }

   vp1.iter().enumerate().for_each(|(i,v)|{ println!("{} {}",i, *v.read().unwrap()); });

}


