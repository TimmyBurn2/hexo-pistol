const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const INITIAL_STATE: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// The SHA-256 of `bytes`, lower-case hex.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    digest.finish_hex()
}

/// A SHA-256 taken over bytes handed to it in pieces.
///
/// The one-shot [`sha256_hex`] is this, fed once — so the two cannot disagree,
/// and the published vectors that pin one pin the other.
///
/// **It exists so a digest over a large payload does not require the payload.**
/// The one-shot form copies its whole input to pad it, and a caller that also
/// has to hold the payload to write it afterwards pays for it twice more; the
/// arena's census artifact is the case that made this worth having, at a
/// design estimate of some gigabytes.
pub struct Sha256 {
    state: [u32; 8],
    /// Bytes not yet in a full block. Never 64 — a full one is compressed and
    /// cleared the moment it fills.
    buffer: [u8; 64],
    buffered: usize,
    /// Bytes fed so far, which is what the padding's length field states.
    length: u64,
}

impl Sha256 {
    /// A digest of nothing.
    pub fn new() -> Sha256 {
        Sha256 {
            state: INITIAL_STATE,
            buffer: [0u8; 64],
            buffered: 0,
            length: 0,
        }
    }

    /// Feed the next piece. Feeding `a` then `b` digests exactly as feeding
    /// `ab` does: where the pieces are cut is not part of the answer.
    pub fn update(&mut self, mut bytes: &[u8]) {
        self.length = self.length.wrapping_add(bytes.len() as u64);
        if self.buffered > 0 {
            let want = (64 - self.buffered).min(bytes.len());
            self.buffer[self.buffered..self.buffered + want].copy_from_slice(&bytes[..want]);
            self.buffered += want;
            bytes = &bytes[want..];
            if self.buffered < 64 {
                // Still short of a block. RETURNING here is load-bearing: the
                // tail below would otherwise overwrite `buffered` with the
                // remainder of an empty slice and drop what is held.
                return;
            }
            let block = self.buffer;
            compress(&mut self.state, &block);
            self.buffered = 0;
        }
        // `as_chunks` rather than `chunks_exact`: the chunk size is a constant,
        // and clippy's `chunks_exact_to_as_chunks` (rustc 1.98) refuses the
        // latter. The tuple is the same pair — whole blocks, then the tail this
        // call keeps for the next one.
        let (blocks, rest) = bytes.as_chunks::<64>();
        for block in blocks {
            compress(&mut self.state, block);
        }
        self.buffer[..rest.len()].copy_from_slice(rest);
        self.buffered = rest.len();
    }

    /// The digest of everything fed, lower-case hex.
    pub fn finish_hex(mut self) -> String {
        let bit_length = self.length.wrapping_mul(8);
        // The padding is fed through `update`'s own path rather than assembled
        // beside it, so there is one block-splitting rule in this file and not
        // two that can come apart.
        // `0x80`, then zeros, then the length: the total is 56 mod 64 before
        // the eight length bytes. Written as an addition because `56 - x` on a
        // `usize` underflows for the very lengths that need the most padding.
        let zeros = (55 + 64 - (self.length as usize % 64)) % 64;
        self.length = 0;
        self.update(&[0x80]);
        self.update(&vec![0u8; zeros]);
        self.update(&bit_length.to_be_bytes());
        let mut hex = String::with_capacity(64);
        for word in self.state {
            for byte in word.to_be_bytes() {
                hex.push_str(&format!("{byte:02x}"));
            }
        }
        hex
    }
}

impl Default for Sha256 {
    fn default() -> Sha256 {
        Sha256::new()
    }
}

/// One 64-byte block into the state.
fn compress(state: &mut [u32; 8], block: &[u8]) {
    let mut schedule = [0u32; 64];
    for (word, source) in schedule.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_be_bytes([source[0], source[1], source[2], source[3]]);
    }
    for index in 16..64 {
        let a = schedule[index - 15];
        let b = schedule[index - 2];
        let s0 = a.rotate_right(7) ^ a.rotate_right(18) ^ (a >> 3);
        let s1 = b.rotate_right(17) ^ b.rotate_right(19) ^ (b >> 10);
        schedule[index] = schedule[index - 16]
            .wrapping_add(s0)
            .wrapping_add(schedule[index - 7])
            .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (constant, word) in ROUND_CONSTANTS.iter().zip(schedule) {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choose = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(choose)
            .wrapping_add(*constant)
            .wrapping_add(word);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(majority);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *slot = slot.wrapping_add(value);
    }
}

#[cfg(test)]
mod tests {
    use super::{Sha256, sha256_hex};

    /// Where the pieces are cut is not part of the answer.
    ///
    /// The published vectors below pin the one-shot form; this pins that the
    /// streaming form agrees with it under every split of the same bytes,
    /// including splits that land inside a block and on its boundary — which is
    /// where a buffer that carried a partial block wrongly would show.
    #[test]
    fn a_streamed_digest_does_not_depend_on_where_the_pieces_were_cut() {
        let payload: Vec<u8> = (0..1000u32).map(|n| (n % 251) as u8).collect();
        let whole = sha256_hex(&payload);
        for cut in [0, 1, 63, 64, 65, 127, 128, 129, 500, 999, 1000] {
            let mut digest = Sha256::new();
            digest.update(&payload[..cut]);
            digest.update(&payload[cut..]);
            assert_eq!(digest.finish_hex(), whole, "cut at {cut}");
        }
        let mut byte_at_a_time = Sha256::new();
        for byte in &payload {
            byte_at_a_time.update(&[*byte]);
        }
        assert_eq!(byte_at_a_time.finish_hex(), whole, "one byte at a time");
    }

    /// The published FIPS 180-4 vectors (docs/decisions.md D-37, D-60). The pins
    /// in this workspace's fixture tests are computed by this function, so an
    /// implementation nobody checked would pin whatever it happened to produce.
    #[test]
    fn sha256_matches_published_test_vectors() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        assert_eq!(
            sha256_hex(&b"a".repeat(1_000_000)),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }
}
