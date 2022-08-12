use std::io::Read;

pub fn read_varint<R>(mut reader: R) -> Vec<u8>
where
    R: Read,
{
    let mut buf_len1 = vec![0u8; 1];
    match reader.read_exact(&mut buf_len1) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };
    let valid_len = match buf_len1[0] {
        0xfd => 2,
        0xfe => 4,
        0xff => 8,
        _ => 0,
    };

    if valid_len > 0 {
        let mut buf = vec![0u8; valid_len];
        match reader.read_exact(&mut buf) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
        buf
    } else {
        buf_len1
    }
}

pub fn encode_varint(i: u128) -> Result<Vec<u8>, String> {
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
