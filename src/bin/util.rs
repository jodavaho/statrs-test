use statrs::statistics::Distribution;
use statrs::distribution::FisherSnedecor;

fn main() {
    let n = FisherSnedecor::new(1.0, 1.0).unwrap();
    assert!(n.variance().is_none());
}