extern crate rand;

extern crate raphy;
use raphy::csr::CSR;

fn main() {

    const NUMV: usize = 50;
    const MAXE: usize = 50;
    let csr = CSR::new(NUMV, CSR::random_el(NUMV, MAXE));
    csr.write_csr_mmap(String::from("/home/blucia/cvsandbox/raphy/test-out.g"));
    let csr2 = CSR::new_mmap(String::from("/home/blucia/cvsandbox/raphy/test-out.g")); 

}


