use std::io::Read;

pub fn read_bytes<R: Read>(mut reader: R, count: usize) -> Result<Vec<u8>, ::std::io::Error> {
    unsafe {
        let mut vec = Vec::with_capacity(count);
        vec.reserve_exact(count);
        vec.set_len(count);
        reader.read_exact(vec.as_mut_slice())?;
        Ok(vec)
    }
}