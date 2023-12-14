use std::io::Cursor;

use bytes::{BytesMut, Buf};
use mini_redis::{Frame, Result};
use tokio::{net::TcpStream, io::AsyncReadExt};

pub struct Connection {
  stream: TcpStream,
  buffer: BytesMut,
  cursor: usize,
}

impl Connection {
  pub fn new(stream: TcpStream) -> Connection {
    Connection {
      stream,
      buffer: BytesMut::with_capacity(4096),
      cursor: 0
    }
  }

  pub async fn read_frame(&mut self)
  -> Result<Option<Frame>>
{
  loop {
      if let Some(frame) = self.parse_frame()? {
          return Ok(Some(frame));
      }

      // Ensure the buffer has capacity
      if self.buffer.len() == self.cursor {
          // Grow the buffer
          self.buffer.resize(self.cursor * 2, 0);
      }

      // Read into the buffer, tracking the number
      // of bytes read
      let n = self.stream.read(
          &mut self.buffer[self.cursor..]).await?;

      if 0 == n {
          if self.cursor == 0 {
              return Ok(None);
          } else {
              return Err("connection reset by peer".into());
          }
      } else {
          // Update our cursor
          self.cursor += n;
      }
  }
}
  fn parse_frame(&mut self)
    -> Result<Option<Frame>>
{
    // Create the `T: Buf` type.
    let mut buf = Cursor::new(&self.buffer[..]);

    // Check whether a full frame is available
    match Frame::check(&mut buf) {
        Ok(_) => {
            // Get the byte length of the frame
            let len = buf.position() as usize;

            // Reset the internal cursor for the
            // call to `parse`.
            buf.set_position(0);

            // Parse the frame
            let frame = Frame::parse(&mut buf)?;

            // Discard the frame from the buffer
            self.buffer.advance(len);

            // Return the frame to the caller.
            Ok(Some(frame))
        }
        // An error was encountered
        Err(e) => Err(e.into()),
    }
}
}