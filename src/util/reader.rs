use std::io::Read;

pub fn read<R>(reader: R, size: usize) -> Vec<u8>
where
    R: Read,
{
    let buf = vec![0u8; size];
    match reader.read_exact(&mut buf) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };
    buf
}
