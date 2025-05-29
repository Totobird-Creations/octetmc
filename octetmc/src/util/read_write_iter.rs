use std::io;


/// A 64-byte buffer which can be read from, written to, or iterated through.
///
/// The 64-bytes are recycled like a tape.
pub struct ReadWriteIter {
    buf        : [u8; 64],
    /// Read head points to the last read position.
    read_head  : usize,
    /// Write head points to the next writing position.
    write_head : usize
}


impl Default for ReadWriteIter {

    fn default() -> Self {
        Self {
            buf        : [0; 64],
            read_head  : 0,
            write_head : 0
        }
    }

}


impl io::Read for ReadWriteIter {

    fn read(&mut self, buf : &mut [u8]) -> io::Result<usize> {
        let head_wrap = self.buf.len();
        for (i, &b,) in buf.iter().enumerate() {
            let next_read_head = (self.read_head + 1).rem_euclid(head_wrap);
            if (next_read_head != self.write_head) {
                self.read_head = next_read_head;
                self.buf[self.read_head] = b;
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }

}


impl io::Write for ReadWriteIter {

    fn write(&mut self, buf : &[u8]) -> io::Result<usize> {
        let head_wrap = self.buf.len();
        for (i, &b,) in buf.iter().enumerate() {
            if (self.write_head != self.read_head) {
                self.buf[self.write_head] = b;
                self.write_head = (self.write_head + 1).rem_euclid(head_wrap);
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }

    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }

}


impl Iterator for ReadWriteIter {

    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        match (io::Read::read(self, &mut buf)) {
            Ok(0) | Err(_) => None,
            _              => Some(buf[0])
        }
    }

}
