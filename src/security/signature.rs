use crate::ecc::s256_field::S256Field;
use crate::ecc::s256_point::S256Point;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Self { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        let marker = 0x30;
        let r_marker = 0x02;
        let (_, mut r_value_bytes) = self.r.to_bytes_be();
        // remove all 0x0 from head
        let mut r_head = r_value_bytes.first().unwrap();
        loop {
            if *r_head == 0u8 {
                r_value_bytes.remove(0);
                r_head = r_value_bytes.first().unwrap();
            } else {
                break;
            }
        }
        // if the first bit of r_value_bytes is 1, add 0x0 to the head
        if r_head & 0x80 == 0x80 {
            r_value_bytes.insert(0, 0x0);
        }
        // !TODO: ここの as u8, 無理がある？丁寧にやるなら r_length の値で場合わけや panic が必要かも。
        let r_length = r_value_bytes.len() as u8;

        let s_marker = 0x02;
        let (_, mut s_value_bytes) = self.s.to_bytes_be();
        let mut s_head = s_value_bytes.first().unwrap();
        loop {
            if *s_head == 0u8 {
                s_value_bytes.remove(0);
                s_head = s_value_bytes.first().unwrap();
            } else {
                break;
            }
        }
        if s_head & 0x80 == 0x80 {
            s_value_bytes.insert(0, 0x0);
        }
        // !TODO: ここの as u8, 無理がある？丁寧にやるなら r_length の値で場合わけや panic が必要かも。
        let s_length = s_value_bytes.len() as u8;
        // two markers + two length parameters + r_length + s_length
        let sig_length = 2 + 2 + r_length + s_length;

        // remove all null bytes from rbin head
        let mut result = Vec::<u8>::new();
        result.push(marker);
        result.push(sig_length);
        result.push(r_marker);
        result.push(r_length);
        result.extend(r_value_bytes);
        result.push(s_marker);
        result.push(s_length);
        result.extend(s_value_bytes);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::hex::hex;

    #[test]
    fn signature_test1() {
        let bytes_z = b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423";
        let z = BigInt::parse_bytes(bytes_z, 16).unwrap();
        let bytes_r = b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6";
        let r = BigInt::parse_bytes(bytes_r, 16).unwrap();
        let bytes_s = b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        let s = BigInt::parse_bytes(bytes_s, 16).unwrap();
        let bytes_px = b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574";
        let px = S256Field::new(BigInt::parse_bytes(bytes_px, 16).unwrap());
        let bytes_py = b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4";
        let py = S256Field::new(BigInt::parse_bytes(bytes_py, 16).unwrap());

        let point = S256Point::new(Some(px), Some(py));
        assert_eq!(true, point.verify(z, Signature::new(r, s)))
    }

    #[test]
    fn signature_test2() {
        let bytes_z = b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60";
        let z = BigInt::parse_bytes(bytes_z, 16).unwrap();
        let bytes_r = b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395";
        let r = BigInt::parse_bytes(bytes_r, 16).unwrap();
        let bytes_s = b"68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4";
        let s = BigInt::parse_bytes(bytes_s, 16).unwrap();
        let bytes_px = b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
        let px = S256Field::new(BigInt::parse_bytes(bytes_px, 16).unwrap());
        let bytes_py = b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
        let py = S256Field::new(BigInt::parse_bytes(bytes_py, 16).unwrap());

        let point = S256Point::new(Some(px), Some(py));
        assert_eq!(true, point.verify(z, Signature::new(r, s)))
    }

    #[test]
    fn signature_test3() {
        let bytes_z = b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let z = BigInt::parse_bytes(bytes_z, 16).unwrap();
        let bytes_r = b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";
        let r = BigInt::parse_bytes(bytes_r, 16).unwrap();
        let bytes_s = b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";
        let s = BigInt::parse_bytes(bytes_s, 16).unwrap();
        let bytes_px = b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
        let px = S256Field::new(BigInt::parse_bytes(bytes_px, 16).unwrap());
        let bytes_py = b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
        let py = S256Field::new(BigInt::parse_bytes(bytes_py, 16).unwrap());

        let point = S256Point::new(Some(px), Some(py));
        assert_eq!(true, point.verify(z, Signature::new(r, s)))
    }
    #[test]
    fn der_test() {
        let bytes_r = b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6";
        let r = BigInt::parse_bytes(bytes_r, 16).unwrap();
        let bytes_s = b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        let s = BigInt::parse_bytes(bytes_s, 16).unwrap();
        let sig = Signature::new(r, s);
        assert_eq!(hex(&sig.der()), "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    }
}
