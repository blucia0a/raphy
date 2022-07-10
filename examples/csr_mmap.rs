extern crate rand;

extern crate raphy;
use raphy::csr::CSR;

fn main() {

    const NUMV: usize = 500;
    const MAXE: usize = 100000;
    let csr = CSR::new(NUMV, CSR::random_el(NUMV, MAXE));
    csr.write_csr_mmap(String::from("./test-out.g"));
    //csr.read_only_scan(|v0, v1| println!("{},{}", v0, v1));

}


