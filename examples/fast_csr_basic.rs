extern crate rand;

extern crate raphy;
use raphy::fast_csr::FastCSR;

fn main() {

    let fcsr = FastCSR::new(String::from("./big.csr")); 
   
    fcsr.read_only_scan(|v,n|{println!("{} --> {}",v,n)});

}


