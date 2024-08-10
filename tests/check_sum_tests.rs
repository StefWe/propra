extern crate propra;
use propra::check_sum::CheckSum;

use std::fs::File;
use std::io::Read;

#[test]
fn check_sum_empty_data() {
    let checksum = CheckSum::new();
    assert_eq!(1, checksum.calc());
}

#[test]
fn check_sum_test() {
    let mut checksum = CheckSum::new();
    let v: Vec<u8> = vec![b't', b'e', b's', b't'];
    checksum.add_vec(v);
    assert_eq!(30016622, checksum.calc());
}

#[test]
fn ke1_03_propra() {
    let mut f = File::open("tests/img/test_03_uncompressed.propra").unwrap();
    let mut buffer: [u8; 9000] = [0; 9000];

    let mut checksum = CheckSum::new();

    // read up to 10 bytes
    let mut n = f.read(&mut buffer[..]).unwrap();
    let mut offset = 30;
    while n > 0 {
        for i in &buffer[offset..n] {
            checksum.add(*i);
        }
        offset = 0;

        n = f.read(&mut buffer[..]).unwrap();
    }
    assert_eq!("0x349797E6", checksum.hex());
}

#[test]
fn ke1_04_propra() {
    let mut f = File::open("tests/img/test_04_uncompressed.propra").unwrap();
    let mut buffer: [u8; 9000] = [0; 9000];

    let mut checksum = CheckSum::new();

    // read up to 10 bytes
    let mut n = f.read(&mut buffer[..]).unwrap();
    let mut offset = 30;
    while n > 0 {
        for i in &buffer[offset..n] {
            checksum.add(*i);
        }
        offset = 0;

        n = f.read(&mut buffer[..]).unwrap();
    }
    assert_eq!("0x280C60A7", checksum.hex());
}
