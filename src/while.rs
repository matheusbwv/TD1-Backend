fn main() {
    for letra in ('A'..='Z').chain('a'..='z'){
        for _ in 0..1 {
            print!("{}", letra);
        }
    }
}