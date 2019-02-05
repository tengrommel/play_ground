fn main() {
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));
    print!("{} {} {}", t.0, t.1, t.2);
    print!("{:?}", f.1);
}
