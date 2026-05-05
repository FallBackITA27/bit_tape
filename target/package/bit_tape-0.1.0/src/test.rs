use crate::BitTape;

#[test]
fn copies() {
    let data: u64 = 0b0111011001101010000111010000101101010101011110101000011010101001;

    let data_bytes = data.to_ne_bytes();
    let byte_reader = BitTape::from(data_bytes);

    assert_eq!(data, unsafe {
        byte_reader.copy_qword(0).unwrap_unchecked()
    });

    assert_eq!(data as u32, unsafe {
        byte_reader.copy_dword(4).unwrap_unchecked()
    });

    assert_eq!((data >> 32) as u32, unsafe {
        byte_reader.copy_dword(0).unwrap_unchecked()
    });

    assert_eq!(data as u16, unsafe {
        byte_reader.copy_word(6).unwrap_unchecked()
    });

    assert_eq!((data >> 48) as u16, unsafe {
        byte_reader.copy_word(0).unwrap_unchecked()
    });

    assert_eq!((data >> 32) as u16, unsafe {
        byte_reader.copy_word(2).unwrap_unchecked()
    });

    assert_eq!(data as u8, unsafe {
        byte_reader.copy_byte(7).unwrap_unchecked()
    });

    for bit in 0..64 {
        let bit_mask = 1 << (63 - bit);
        let bit_bool_value = (data & bit_mask) != 0;
        assert_eq!(bit_bool_value, unsafe {
            byte_reader.read_bool(bit).unwrap_unchecked()
        })
    }
}

#[test]
fn shift_right() {
    let data: u64 = 0b0111011001101010000111010000101101010101011110101000011010101001;
    let data_bytes = data.to_ne_bytes();
    let byte_reader = BitTape::from(data_bytes);

    let mut shift = byte_reader.clone();
    shift.shift_right(4);
    assert_eq!(data >> 4, unsafe { shift.copy_qword(0).unwrap_unchecked() });

    let mut shift = byte_reader.clone();
    shift.shift_right(15);
    assert_eq!(data >> 15, unsafe {
        shift.copy_qword(0).unwrap_unchecked()
    });
}

#[test]
fn shift_left() {
    let data: u64 = 0b0111011001101010000111010000101101010101011110101000011010101001;
    let data_bytes = data.to_ne_bytes();
    let byte_reader = BitTape::from(data_bytes);

    let mut shift = byte_reader.clone();
    shift.shift_left(4);
    assert_eq!(data << 4, unsafe { shift.copy_qword(0).unwrap_unchecked() });

    let mut shift = byte_reader.clone();
    shift.shift_left(15);
    assert_eq!(data << 15, unsafe {
        shift.copy_qword(0).unwrap_unchecked()
    });
}
