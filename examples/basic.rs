use metaphone3_sys::metaphone3;

fn main() {
    let word = "SMITH";
    let (primary, secondary) = metaphone3(word, false, false).unwrap();
    println!("Primary: {}, Secondary: {}", primary, secondary);
}