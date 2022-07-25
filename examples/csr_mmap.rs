extern crate rand;

extern crate raphy;
use raphy::csr::CSR;

fn main() {
    let csr = CSR::new_from_el_mmap(1000000,String::from("graphs/large.el"));
    csr.write_fastcsr(String::from("graphs/large.csr"));
}
