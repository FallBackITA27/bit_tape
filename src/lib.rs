#[cfg(test)]
mod test;

#[derive(thiserror::Error, Debug)]
pub enum BitTapeError {
    #[error("Conversion Error: {0}")]
    ConversionError(#[from] ConversionError),
    #[error("Read Error: {0}")]
    ReadError(#[from] ReadError),
}

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("Couldn't convert type to BitTape, too short")]
    TooShort,
}

#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    #[error("Index out of bounds")]
    OutOfBoundsIndex,
}

/// Stores the number as a sort of tape of bits from left to right, dealt with as big endian.
#[derive(Clone)]
pub struct BitTape<const N: usize>([u8; N]);

impl<const N: usize> BitTape<N> {
    /// Gets length in bits (const param N << 3)
    pub const fn bit_len(&self) -> usize {
        N << 3
    }

    /// Gets length (const param N)
    pub const fn byte_len(&self) -> usize {
        N
    }

    /// Gets length in words (const param N >> 1)
    pub const fn word_len(&self) -> usize {
        N >> 1
    }

    /// Gets length in dwords (const param N >> 2)
    pub const fn dword_len(&self) -> usize {
        N >> 2
    }

    /// Gets length in qwords (const param N >> 4)
    pub const fn qword_len(&self) -> usize {
        N >> 4
    }

    /// Copies qword, identical to indexing a range i..i+8
    pub fn copy_qword(&self, idx: usize) -> Result<u64, BitTapeError> {
        if idx > self.byte_len() - 8 {
            return Err(BitTapeError::ReadError(ReadError::OutOfBoundsIndex));
        }

        let bytes = &self.0[idx..idx + 8];
        let num = u64::from_be_bytes(unsafe { bytes.try_into().unwrap_unchecked() });
        Ok(num)
    }

    /// Copies dword, identical to indexing a range i..i+4
    pub fn copy_dword(&self, idx: usize) -> Result<u32, BitTapeError> {
        if idx > self.byte_len() - 4 {
            return Err(BitTapeError::ReadError(ReadError::OutOfBoundsIndex));
        }

        let bytes = &self.0[idx..idx + 4];
        let num = u32::from_be_bytes(unsafe { bytes.try_into().unwrap_unchecked() });
        Ok(num)
    }

    /// Copies dword, identical to indexing a range i..i+2
    pub fn copy_word(&self, idx: usize) -> Result<u16, BitTapeError> {
        if idx > self.byte_len() - 2 {
            return Err(BitTapeError::ReadError(ReadError::OutOfBoundsIndex));
        }

        let bytes = &self.0[idx..idx + 2];
        let num = u16::from_be_bytes(unsafe { bytes.try_into().unwrap_unchecked() });
        Ok(num)
    }

    /// Copies byte, identical to indexing
    pub const fn copy_byte(&self, idx: usize) -> Result<u8, BitTapeError> {
        if idx > self.byte_len() {
            return Err(BitTapeError::ReadError(ReadError::OutOfBoundsIndex));
        }

        Ok(self.0[idx])
    }

    pub fn shift_right(&mut self, d: usize) {
        if d > self.bit_len() {
            self.0.fill(0);
            return;
        }

        let temp_clone = self.0;
        let byte_shift = d / 8;
        let bit_shift = d % 8;
        let mut spare_bits = 0;

        for i in 0..N {
            let replace_byte = match i < byte_shift {
                true => 0,
                false => temp_clone[i - byte_shift],
            };

            let tmp_spare_bits = replace_byte << (8 - bit_shift);
            self.0[i] = (replace_byte >> bit_shift) | spare_bits;
            spare_bits = tmp_spare_bits;
        }
    }

    pub fn shift_left(&mut self, d: usize) {
        if d > self.bit_len() {
            self.0.fill(0);
            return;
        }

        let temp_clone = self.0;
        let byte_shift = d / 8;
        let bit_shift = d % 8;
        let mut spare_bits = 0;

        for i in (0..N).rev() {
            let replace_byte = match (i + byte_shift) >= N {
                true => 0,
                false => temp_clone[i + byte_shift],
            };

            let tmp_spare_bits = replace_byte >> (8 - bit_shift);
            self.0[i] = (replace_byte << bit_shift) | spare_bits;
            spare_bits = tmp_spare_bits;
        }
    }

    pub const fn read_bool(&self, idx: usize) -> Result<bool, BitTapeError> {
        if idx > self.bit_len() {
            return Err(BitTapeError::ReadError(ReadError::OutOfBoundsIndex));
        }

        Ok(self.0[idx / 8] & (0b10000000 >> ((idx as u8) % 8)) != 0)
    }
}

impl<const N: usize> From<[u8; N]> for BitTape<N> {
    fn from(value: [u8; N]) -> Self {
        let value = if cfg!(target_endian = "little") {
            let mut new_value = [0; N];
            for i in 0..N {
                new_value[i] = value[N - i - 1];
            }
            new_value
        } else {
            value
        };

        Self(value)
    }
}

impl<const N: usize> TryFrom<&[u8]> for BitTape<N> {
    type Error = BitTapeError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < N {
            return Err(BitTapeError::ConversionError(ConversionError::TooShort));
        }

        let value = unsafe { TryInto::<[u8; N]>::try_into(&value[0..N]).unwrap_unchecked() };
        Ok(Self::from(value))
    }
}

impl From<u128> for BitTape<16> {
    fn from(value: u128) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<i128> for BitTape<16> {
    fn from(value: i128) -> Self {
        Self(value.to_be_bytes())
    }
}


impl From<u64> for BitTape<8> {
    fn from(value: u64) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<i64> for BitTape<8> {
    fn from(value: i64) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<f64> for BitTape<8> {
    fn from(value: f64) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<u32> for BitTape<4> {
    fn from(value: u32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<i32> for BitTape<4> {
    fn from(value: i32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<f32> for BitTape<4> {
    fn from(value: f32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<u16> for BitTape<2> {
    fn from(value: u16) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<i16> for BitTape<2> {
    fn from(value: i16) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<u8> for BitTape<1> {
    fn from(value: u8) -> Self {
        Self([value])
    }
}

impl From<i8> for BitTape<1> {
    fn from(value: i8) -> Self {
        Self([value.cast_unsigned()])
    }
}
