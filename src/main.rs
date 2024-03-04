use color_eyre::Result;
use zerocopy::LittleEndian;
use hex_slice::AsHex;

fn main() {
    let bytes = std::fs::read("user.o").expect("Failed opening user program !");
    println!("{:x}", bytes.as_hex());
    let reader = Reader::new(bytes);

}

/// Fasterthanlime's reader in https://fasterthanli.me/series/reading-files-the-hard-way/part-3#reading-an-inode-in-ext4
struct Reader<IO> {
    inner: IO,
}

impl<IO: positioned_io::ReadAt> Reader<IO> {
    fn new(inner: IO) -> Self {
        Self { inner }
    }

    fn u16(&self, offset: u64) -> Result<u16> {
        let mut cursor = positioned_io::Cursor::new_pos(&self.inner, offset);
        use byteorder::ReadBytesExt;
        Ok(cursor.read_u16::<LittleEndian>()?)
    }
}
