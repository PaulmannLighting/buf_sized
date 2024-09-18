use buf_sized::buf_size;
use buf_sized_derive::BufSized;

#[derive(BufSized)]
struct Data {
    header: u8,
    num: u128,
    payload: [u8; 12],
    crc: u32,
}

#[test]
fn test_struct() {
    assert_eq!(buf_size::<Data>(), 1 + 16 + 12 + 4);
}
