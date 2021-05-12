use statrs::distribution::{Exp, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;


fn main() {
    let n = Exp::new(1.0).unwrap();
    assert_eq!(n.mean(), Some(1.0));
    assert_eq!(n.variance(), Some(1.0));
    assert_eq!(n.entropy(), Some(1.0));
    assert_eq!(n.skewness(), Some(2.0));
    assert_eq!(n.cdf(1.0), 0.6321205588285576784045);
    assert_eq!(n.pdf(1.0), 0.3678794411714423215955)
}