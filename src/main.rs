fn main() {
    let num = 100;
    println!("{}", num as u8 as char);

    // chars are all 4 bytes because it's possible for a byte to be 4 bytes, but usually it's just 1 byte for normal english chars
    let slice = "hello";
    println!("bytes {}", slice.len()); // num of bytes
    println!("bytes {}", slice.chars().count()); // readable chars

    let my_float = 5.;
    dbg!(my_float);

}

#[test]
fn basics() {
    assert!(1 == 1);
}
