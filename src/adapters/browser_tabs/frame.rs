// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Native messaging frames: a native-endian `u32` length, then JSON.

/// A length of zero or over the cap. The stream must be closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadFrame;

/// Collects bytes and yields whole frames.
#[derive(Debug)]
pub struct FrameReader {
    buf: Vec<u8>,
    cap: usize,
    dead: bool,
}

impl FrameReader {
    /// `cap` is the largest accepted payload, in bytes.
    #[must_use]
    pub fn new(cap: usize) -> Self {
        Self {
            buf: Vec::new(),
            cap,
            dead: false,
        }
    }

    /// Adds `data` and returns every frame now complete.
    ///
    /// # Errors
    ///
    /// Returns an error when a length is `0` or over the cap. Later
    /// calls keep failing.
    pub fn push(&mut self, data: &[u8]) -> Result<Vec<Vec<u8>>, BadFrame> {
        if self.dead {
            return Err(BadFrame);
        }
        self.buf.extend_from_slice(data);

        let mut out = Vec::new();
        let mut at = 0;

        while let Some(head) = self.buf.get(at..at + 4) {
            let len = u32::from_ne_bytes([head[0], head[1], head[2], head[3]]);
            let len = usize::try_from(len).unwrap_or(usize::MAX);

            if len == 0 || len > self.cap {
                self.dead = true;
                self.buf = Vec::new();
                return Err(BadFrame);
            }
            let Some(body) = self.buf.get(at + 4..at + 4 + len) else {
                break;
            };

            out.push(body.to_vec());
            at += 4 + len;
        }
        self.buf.drain(..at); // One move for every frame taken
        Ok(out)
    }
}

/// Prefixes `payload` with its length. Payloads stay far below 4 GiB.
#[must_use]
pub fn encode(payload: &[u8]) -> Vec<u8> {
    let len = u32::try_from(payload.len()).unwrap_or(u32::MAX);
    let mut out = Vec::with_capacity(4 + payload.len());

    out.extend_from_slice(&len.to_ne_bytes());
    out.extend_from_slice(payload);
    out
}

#[cfg(test)]
mod tests {
    use super::{FrameReader, encode};

    #[test]
    fn splits_and_joins_frames() {
        let mut reader = FrameReader::new(8);
        let mut both = encode(b"ab");

        both.extend(encode(b"cd"));
        assert_eq!(reader.push(&both[..3]).unwrap(), Vec::<Vec<u8>>::new());
        let frames = reader.push(&both[3..]).unwrap();

        assert_eq!(frames, [b"ab".to_vec(), b"cd".to_vec()]);
    }

    #[test]
    fn rejects_empty_and_over_cap_for_good() {
        let mut reader = FrameReader::new(4);

        assert!(reader.push(&encode(b"")).is_err());
        assert!(reader.push(&encode(b"ok")).is_err());
        assert!(FrameReader::new(2).push(&encode(b"abcd")).is_err());
    }
}
