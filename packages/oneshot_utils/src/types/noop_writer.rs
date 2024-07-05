use std::io::{self, Write};

pub struct NoopWriter;

impl Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Pretend we wrote everything successfully
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // No-op for flush as well
        Ok(())
    }
}
