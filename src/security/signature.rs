use crate::ecc::field_element::FieldElement;
use crate::ecc::point::{Point, N};
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Signature {
    r: BigInt,
    s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Self { r, s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_test() {
        // !TODO: なんか知らんけどバカ遅い！改善必須
        let n = N::new().value;

        let bytes_z = b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423";
        let z = FieldElement::new(BigInt::parse_bytes(bytes_z, 16).unwrap(), n.clone());
        let bytes_r = b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6";
        let r = FieldElement::new(BigInt::parse_bytes(bytes_r, 16).unwrap(), n.clone());
        let bytes_s = b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        let s = FieldElement::new(BigInt::parse_bytes(bytes_s, 16).unwrap(), n.clone());
        let bytes_px = b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574";
        let px = FieldElement::new_s256(BigInt::parse_bytes(bytes_px, 16).unwrap());
        let bytes_py = b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4";
        let py = FieldElement::new_s256(BigInt::parse_bytes(bytes_py, 16).unwrap());

        let point = Point::new_s256(Some(px), Some(py));
        let s_inv = FieldElement::pow(s, n - BigInt::from(2u8));
        let u = &z * &s_inv;
        let v = &r * &s_inv;

        assert_eq!(
            true,
            (Point::new_g().rmul_s256(u.num) + point.rmul_s256(v.num))
                .x
                .unwrap()
                .num
                == r.num
        )
    }
}
