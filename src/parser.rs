pub struct Parser {
    buf: Vec<u8>,
    pos: usize,
}

impl Parser {
    fn new(buf: Vec<u8>) -> Self {
        Self { buf, pos: 0 }
    }

    fn read_u8(&mut self) -> u8 {
        let byte = self.buf[self.pos];
        self.pos += 1;
        byte
    }

    fn read_u16(&mut self) -> u16 {
        let bytes = [self.buf[self.pos], self.buf[self.pos + 1]];
        self.pos += 2;
        u16::from_be_bytes(bytes)
    }

    fn read_u32(&mut self) -> u32 {
        let bytes = [
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ];
        self.pos += 4;
        u32::from_be_bytes(bytes)
    }
}
