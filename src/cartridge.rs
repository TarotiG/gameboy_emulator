struct RomReader<'a> {
    data: &'a [u8],
    cursor_pos: usize
}

impl<'a> RomReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        RomReader { data, cursor_pos: 0 }
    }

    fn read_byte(&mut self) -> Option<u8> {

        if self.cursor_pos < self.data.len() {
            let byte = self.data[self.cursor_pos as usize];
            self.cursor_pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn read_slice(&mut self, len: usize) -> Option<&'a [u8]> {
        let end = self.cursor_pos + len;

        if end > self.data.len() {
            None
        } else {
            let slice = &self.data[self.cursor_pos..end];
            self.cursor_pos = end;
            Some(slice)
        }
    }
}

fn find_header<'a>(rom: &'a [u8]) -> Option<&'a [u8]> {
    if rom.len() >= 0x134 {
        Some(&rom[0x104..0x134])
    } else {
        None
    }
}
