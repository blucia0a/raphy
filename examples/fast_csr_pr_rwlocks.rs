extern crate rand;

extern crate raphy;

use rayon::prelude::*;
use raphy::fast_csr::FastCSR;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    const NUMITERS: usize = 10;
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let fcsr = FastCSR::new(String::from("graphs/large.csr"));
    let setup = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let mut vp1 = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        vp1.push(RwLock::new(0.0));
    }
    

    let iters = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut sum_iters = 0;
    for _ in 0..NUMITERS {

        let mut vp2 = Vec::with_capacity(fcsr.getv());
        for _ in 0..fcsr.getv() {
            vp2.push(RwLock::new(0.0));
        }

        let vf = |v: usize, nei: &[usize]| {
            const D: f64 = 0.85;
            let mut n_upd: f64 = 0.0;

            nei.iter()
                .for_each(|v1| n_upd = n_upd + *vp1[*v1].read().unwrap() / (nei.len() as f64));

            {
                let mut prop = vp2[v].write().unwrap();
                *prop = (1.0 - D) / (fcsr.getv() as f64) + D * n_upd;
            }
        };

        let iter_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        fcsr.neighbor_scan(vf);
        vp1.par_iter_mut().enumerate().for_each(|(i,v)|{
            *v.write().unwrap() = *vp2[i].read().unwrap();
        });

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
