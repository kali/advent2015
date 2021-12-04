extern crate md5;

fn main() {
    let input = "bgvyzdsv";
    for i in 0.. {
        let secret = format!("{}{}", input, i);
        let hash = md5::compute(secret);
        let formatted = format!("{:?}", hash);
        if &formatted.as_bytes()[0..6] == b"000000" {
            println!("{}", i);
            break;
        }
    }
}
