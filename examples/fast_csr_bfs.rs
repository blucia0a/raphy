extern crate rand;

extern crate raphy;

use raphy::fast_csr::FastCSR;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicBool, Ordering};
use rayon::prelude::*;

fn main() {

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let fcsr = FastCSR::new(String::from("graphs/large.csr"));
    let setup = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let mut frontier = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        frontier.push(false);
    }
    frontier[0] = true; 
    
    
    let mut visited = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        visited.push(AtomicBool::new(false));
    }

    let iters = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let mut level: u64 = 0;
    loop{


        let changed = AtomicBool::new(false);        
        let mut next_frontier = Vec::with_capacity(fcsr.getv());
        for _ in 0..fcsr.getv() {
            next_frontier.push(AtomicBool::new(false));
        }

        print!("Level {}: [",level);
        frontier
        .par_iter()
        .enumerate()
        .filter(|&(i,v)|{*v == true && !visited[i].load(Ordering::Relaxed)})
        .for_each(|(i,_)|{
            print!("{} ",i);
            fcsr.neighbors(i).par_iter().for_each(|n|{
                visited[i].store(true,Ordering::Relaxed);
                next_frontier[*n].store(true,Ordering::Relaxed);
            });
            changed.store(true,Ordering::Relaxed);
        });
      
        println!("]");
        level = level + 1;
        if !changed.load(Ordering::Relaxed) { 
            break; 
        } 

        frontier.par_iter_mut().enumerate().for_each(|(i,v)|{
          *v = next_frontier[i].load(Ordering::Relaxed);
        });
 
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    println!("CSR Build Time: {}ms",setup - start);
    println!("Total Iters Time: {}ms",end - iters);

/*    vp1.iter().enumerate().for_each(|(i, v)| {
        println!("{} {}", i, *v.read().unwrap());
    });
*/
}
