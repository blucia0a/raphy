extern crate rand;

extern crate raphy;
use raphy::csr::CSR;

fn main() {
    const NUMV: usize = 5;
    const MAXE: usize = 5;
    let csr = CSR::new(NUMV, CSR::random_el(NUMV, MAXE));
    csr.write_fastcsr(String::from("/home/blucia/cvsandbox/raphy/test-out.g"));
}
