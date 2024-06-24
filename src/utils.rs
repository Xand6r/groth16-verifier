use std::mem;

#[derive(Debug)]
struct Uint256([u64; 1]);

impl Uint256 {
    fn new(input: u64) -> Self {
        Uint256([input])
    }

    fn reverse_byte_order(&mut self) {
        let mut reversed = [0u64; 4];

        // Swap bytes
        for i in 0..4 {
            reversed[i] = (((self.0[i] & 0xFF00FF00FF00FF00) >> 8) | ((self.0[i] & 0x00FF00FF00FF00FF) << 8));
        }

        // Swap 2-byte long pairs
        for i in 0..2 {
            reversed[2 * i] = (((reversed[2 * i] & 0xFFFF0000FFFF0000) >> 16) | ((reversed[2 * i] & 0x0000FFFF0000FFFF) << 16));
            reversed[2 * i + 1] = (((reversed[2 * i + 1] & 0xFFFF0000FFFF0000) >> 16) | ((reversed[2 * i + 1] & 0x0000FFFF0000FFFF) << 16));
        }

        // Swap 4-byte long pairs
        for i in 0..2 {
            reversed[i] = ((reversed[i] & 0xFFFFFFFF00000000FFFFFFFF) >> 32) | ((reversed[i] & 0x00000000FFFFFFFF0000) << 32);
        }

        // Swap 8-byte long pairs
        reversed[0] = ((reversed[0] & 0xFFFFFFFFFFFFFFFF0000000000000000) >> 64) | ((reversed[0] & 0x0000000000000000FFFFFFFFFFFFFFFF) << 64);

        // Swap 16-byte long pairs
        reversed[0] = (reversed[0] >> 128) | (reversed[0] << 128);

        self.0 = reversed;
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_reverse_byte(){
        println!("working")
    }
}
