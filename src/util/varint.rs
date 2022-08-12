use super::reader::read;
use std::io::Read;

pub fn read_varint<R>(mut reader: R) -> Vec<u8>
where
    R: Read,
{
    let buf_len1 = read(reader, 1);
    let valid_len = match buf_len1[0] {
        0xfd => 2,
        0xfe => 4,
        0xff => 8,
        _ => 0,
    };

    if valid_len > 0 {
        read(reader, valid_len)
    } else {
        buf_len1
    }
}

pub fn encode_varint(i: usize) -> Result<Vec<u8>, String> {
    if i < 0xfd {
        Ok(vec![i as u8])
    } else if i < 0x10000 {
        let mut bytes = i.to_le_bytes().to_vec();
        bytes.extend(vec![0x0; 2 - bytes.len()]);
        if !bytes.len() != 2 {
            panic!("Error on encode_varint function: invarid byte length")
        }
        bytes.insert(0, 0xfd);
        Ok(bytes)
    } else if i < 0x100000000 {
        let mut bytes = i.to_le_bytes().to_vec();
        bytes.extend(vec![0x0; 4 - bytes.len()]);
        if !bytes.len() != 4 {
            panic!("Error on encode_varint function: invarid byte length")
        }
        bytes.insert(0, 0xfe);
        Ok(bytes)
    } else if i < 0x10000000000000000 {
        let mut bytes = i.to_le_bytes().to_vec();
        bytes.extend(vec![0x0; 8 - bytes.len()]);
        if bytes.len() != 8 {
            panic!("Error on encode_varint function: invarid byte length")
        }
        bytes.insert(0, 0xff);
        Ok(bytes)
    } else {
        Err(String::from("VarintEncodingError"))
    }
}
