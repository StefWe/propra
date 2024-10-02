/*
*   P = An * 2^16 + Bn
*
*   An = [Sum(i+data[i])] mod X | i=1 bis n
*
*   B0 = 1
*   Bi = [Bi-1 + Ai] mod X   für alle i {1,2,...,n}
*
*   mit X = 65521 und n = Länge des Datensegments
*
*   Mit data[i] wird das i-te Byte aus dem Datensegment gelesen.
*   Das erste Byte wird im 1 und nicht mit 0 addressiert.
*/

pub struct CheckSum {
    i: u32,
    an: u32,
    bn: u32,
}

impl Default for CheckSum {
    fn default() -> Self {
        CheckSum::new()
    }
}

impl CheckSum {
    pub fn new() -> Self {
        CheckSum { i: 1, an: 0, bn: 1 }
    }

    #[allow(dead_code)]
    pub fn hex(&self) -> String {
        let result = self.calc();
        format!("{:#010X}", result)
    }

    pub fn calc(&self) -> u32 {
        if self.i == 1 {
            return 1;
        }

        self.an * u32::pow(2, 16) + self.bn
    }

    #[allow(dead_code)]
    pub fn add_vec(&mut self, data: Vec<u8>) {
        if data.is_empty() {
            return;
        }
        for i in data {
            self.add(i);
        }
    }

    pub fn add(&mut self, data: u8) {
        self.an = (self.an + self.an_calc(data)) % 65521;
        self.bn_calc();
        self.i = self.i.wrapping_add(1);
    }

    fn an_calc(&self, data: u8) -> u32 {
        self.i + data as u32
    }

    fn bn_calc(&mut self) {
        self.bn = (self.bn + self.an) % 65521;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_an_calc() {
        let checksum = CheckSum::new();
        assert_eq!(42, checksum.an_calc(41));
    }

    #[test]
    fn check_sum_empty_data() {
        let checksum = CheckSum::new();
        assert_eq!("0x00000001", checksum.hex());
    }

    #[test]
    fn check_sum_zero() {
        let mut checksum = CheckSum::new();
        checksum.add_vec(vec![0]);
        assert_eq!("0x00010002", checksum.hex());
    }

    #[test]
    fn check_sum_one() {
        let mut checksum = CheckSum::new();
        checksum.add_vec(vec![1]);
        assert_eq!("0x00020003", checksum.hex());
    }

    #[test]
    fn check_sum_zero_one() {
        let mut checksum = CheckSum::new();
        checksum.add_vec(vec![0, 1]);
        assert_eq!("0x00040006", checksum.hex());
    }

    #[test]
    fn check_sum_one_zero() {
        let mut checksum = CheckSum::new();
        checksum.add_vec(vec![1, 0]);
        assert_eq!("0x00040007", checksum.hex());
    }

    #[test]
    fn check_sum_255_128() {
        let mut checksum = CheckSum::new();
        checksum.add_vec(vec![255, 128]);
        assert_eq!("0x01820283", checksum.hex());
    }

    #[test]
    fn check_sum_t() {
        let mut checksum = CheckSum::new();
        let v: Vec<u8> = vec![b't'];
        checksum.add_vec(v);
        assert_eq!("0x00750076", checksum.hex());
    }

    #[test]
    fn check_sum_te() {
        let mut checksum = CheckSum::new();
        let v: Vec<u8> = vec![b't', b'e'];
        checksum.add_vec(v);
        assert_eq!("0x00DC0152", checksum.hex());
    }

    #[test]
    fn check_sum_tes() {
        let mut checksum = CheckSum::new();
        let v: Vec<u8> = vec![b't', b'e', b's'];
        checksum.add_vec(v);
        assert_eq!("0x015202A4", checksum.hex());
    }

    #[test]
    fn check_sum_test() {
        let mut checksum = CheckSum::new();
        let v: Vec<u8> = vec![b't', b'e', b's', b't'];
        checksum.add_vec(v);
        assert_eq!("0x01CA046E", checksum.hex());
    }

    #[test]
    fn check_sum_lorem() {
        let mut checksum = CheckSum::new();
        let s:String = "Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string();
        let mut v: Vec<u8> = Vec::new();

        for c in s.chars() {
            v.push(c as u8);
        }
        checksum.add_vec(v);
        assert_eq!("0x3C4EEB4C", checksum.hex());
    }

    #[test]
    fn check_sum_ipsum() {
        let mut checksum = CheckSum::new();
        let s:String = "Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat cupiditat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
        let mut v: Vec<u8> = Vec::new();

        for c in s.chars() {
            v.push(c as u8);
        }
        checksum.add_vec(v);
        assert_eq!("0x079ED65E", checksum.hex());
    }
}
