extern crate rand;

extern crate raphy;

use raphy::fast_csr::FastCSR;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    const NUMITERS: usize = 10;
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let fcsr = FastCSR::new(String::from("./large.csr"));
    let setup = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let mut vp1 = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        vp1.push(0.0);
    }
    
    let mut vp2 = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        vp2.push(0.0);
    }

    let iters = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut sum_iters = 0;
    for _ in 0..NUMITERS {


        let vf = |_v: usize, nei: &[usize]| -> f64{
        
            let mut s = 0; 
            nei.iter().for_each(|n| s=s+n);
            s  as f64
        };

        let iter_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        fcsr.neighbor_scan_prop(vf,&mut vp2);
        vp1.clone_from_slice(&vp2);

        let iter_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        sum_iters = sum_iters + (iter_end - iter_start);
        /*for v in 0..(vp1.len() - 1) {
            *vp1[v].write().unwrap() = *vp2[v].read().unwrap();
        }*/
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    println!("CSR Build Time: {}ms",setup - start);
    println!("Total Iters Time: {}ms",end - iters);
    println!("Average Iter Time: {}ms",sum_iters as f64 / NUMITERS as f64);

/*    vp1.iter().enumerate().for_each(|(i, v)| {
        println!("{} {}", i, *v.read().unwrap());
    });
*/
}
