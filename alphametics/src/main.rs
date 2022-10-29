use itertools::Itertools;


fn main() {
    let it = (0..3).permutations(3);
    for n in it {
        println!("{:?}", n);
    }
}