extern crate rand;

extern crate raphy;
use raphy::csr::CSR;

fn main() {
    let csr = CSR::new_from_el_mmap(10000000,String::from("large.el"));
    csr.write_fastcsr(String::from("large.csr"));
}
