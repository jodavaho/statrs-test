use statrs::distribution::Exp;
use rand::distributions::Distribution;

fn main() {

    let mut r = rand::rngs::OsRng;
    let n = Exp::new(0.5).unwrap();
    print!("{}", n.sample(&mut r));
  
}
