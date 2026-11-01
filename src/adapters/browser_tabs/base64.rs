// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Base64 decoder. No extra crate: the alphabet is fixed.

/// Padding marker in a decoded quantum.
const PAD: u8 = 64;

/// Decodes standard padded base64. Whitespace, and padding anywhere
/// but at the end, are rejected.
#[must_use]
pub fn decode(text: &str) -> Option<Vec<u8>> {
    let (chunks, rest) = text.as_bytes().as_chunks::<4>();

    if !rest.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(chunks.len() * 3);
    let last = chunks.len();

    for (i, chunk) in chunks.iter().enumerate() {
        let mut quantum = [0u8; 4];

        for (slot, &b) in quantum.iter_mut().zip(chunk) {
            *slot = value(b)?;
        }
        let pad = quantum.iter().rev().take_while(|&&v| v == PAD).count();

        if pad > 2 || quantum[..4 - pad].contains(&PAD) {
            return None;
        }
        if pad > 0 && i + 1 != last {
            return None;
        }
        let v = |i: usize| u32::from(quantum[i] % PAD);
        let n = (v(0) << 18) | (v(1) << 12) | (v(2) << 6) | v(3);

        out.extend_from_slice(&n.to_be_bytes()[1..4 - pad]);
    }
    Some(out)
}

/// Value of one base64 character, `PAD` for `=`.
fn value(b: u8) -> Option<u8> {
    Some(match b {
        b'A'..=b'Z' => b - b'A',
        b'a'..=b'z' => b - b'a' + 26,
        b'0'..=b'9' => b - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        b'=' => PAD,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn known_vectors() {
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("YQ==").unwrap(), b"a");
        assert_eq!(decode("YWI=").unwrap(), b"ab");
        assert_eq!(decode("YWJj").unwrap(), b"abc");
        assert_eq!(decode("YWJjYQ==").unwrap(), b"abca");
    }

    #[test]
    fn rejects_bad_input() {
        assert!(decode("YQ=").is_none());
        assert!(decode("Y Q==").is_none());
        assert!(decode("YQ==YWJj").is_none());
        assert!(decode("Y===").is_none());
        assert!(decode("Y=Q=").is_none());
    }
}
