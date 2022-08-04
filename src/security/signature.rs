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

    // !TODO: 工事中
    // pub fn der(&self) -> Vec<u8> {
    //     let (sign, rbin) = self.r.to_bytes_be();
    //     // remove all null bytes from rbin head
    //     for s in rbin {
    //         if s != b'\x00' {
    //             break;
    //         }
    //         rbin.remove(0);
    //     }
    //     if rbin.first().unwrap() {
    //         rbin.insert(0, b'\x00');
    //     }
    //     return rbin;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
