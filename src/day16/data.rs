use std::ops::Range;

pub(super) fn parse_hex(s: &str) -> Vec<u64> {
    let len = s.len();
    let word_count = if len % 16 == 0 {
        len / 16
    } else {
        (len / 16) + 1
    };
    let mut data = Vec::new();
    for w in 0..word_count {
        let first = w * 16;
        let end = if first + 16 > len { len } else { first + 16 };
        let current = &s[first..end];
        let word_value = if current.len() == 16 {
            u64::from_str_radix(current, 16).unwrap()
        } else {
            let mut padded = String::from(current);
            while padded.len() < 16 {
                padded.push('0');
            }
            u64::from_str_radix(&padded, 16).unwrap()
        };

        data.push(word_value);
    }
    data
}

pub(super) fn take_bits(u64s: &[u64], range: Range<usize>) -> Option<u64> {
    let bit_count = range.end - range.start;
    debug_assert!(bit_count <= 64);
    let start_word = range.start / 64;
    let end_word = (range.end - 1) / 64;
    if start_word != end_word {
        let left_side_start = range.start;
        let left_side_end = (start_word + 1) * 64;
        let left_side_value = take_bits(u64s, left_side_start..left_side_end).unwrap();
        // println!("{:#064b} - left side", left_side_value);
        let right_side_start = left_side_end;
        let right_side_end = range.end;
        let right_side_value =
            take_bits(u64s, right_side_start..right_side_end).unwrap_or_default();
        // println!("{:#064b} - right side", right_side_value);
        let right_side_bit_count = right_side_end - right_side_start;
        let mut answer = left_side_value;
        answer <<= right_side_bit_count;
        // println!("{:#064b} - answer shifted", answer);
        answer |= right_side_value;
        // println!("{:#064b} - final answer", answer);
        Some(answer)
    } else {
        let word = match u64s.get(start_word) {
            None => return None,
            Some(x) => *x,
        };
        let range_end_in_word = range.end % 64;
        let shift_amount = if range_end_in_word == 0 {
            0
        } else {
            64 - range_end_in_word
        };
        // // FIXME - wtf?
        // let shift_amount = if end_shift == 0 { 0 } else { end_shift - 0 };

        let shifted = word >> shift_amount;
        let mask = u64::MAX >> 64 - bit_count;
        // println!("{:#064b} - word", word);
        // println!("{:?}", range);
        // println!("{} - shift amount", shift_amount);
        // println!("{:#064b} - shifted", shifted);
        // println!("{:#064b} - mask", mask);
        let answer = shifted & mask;
        // println!("{:#064b} - answer", answer);
        Some(answer)
    }
}

#[test]
fn take_bits_1() {
    let vec: Vec<u64> = vec![0b1011011000000000_0000000000000000_0000000000000000_0000000000000000];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000101;
    let actual = take_bits(&vec, 3..6).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn take_bits_2() {
    let vec: Vec<u64> = vec![
        0b1001110000000000_0000000000000000_0000000000000000_0000000000001011,
        0b1101100000000000_0000000000000000_0000000000000001_0000000000000000,
    ];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000010111101;
    let actual = take_bits(&vec, 60..68).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn take_bits_3() {
    let vec: Vec<u64> = vec![0b1001010000000000_0000000000000000_0000000000000000_000000000000011];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000001;
    let actual = take_bits(&vec, 63..64).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn take_bits_4() {
    let vec: Vec<u64> = vec![0b1001010000000000_0000000000000000_0000000000000000_000000000000111];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000011;
    let actual = take_bits(&vec, 62..64).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn take_bits_5() {
    let vec: Vec<u64> = vec![0b1001010000000000_0000000000000000_0000000000000000_000000000001100];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000100;
    let actual = take_bits(&vec, 61..64).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn take_bits_6() {
    let vec: Vec<u64> = vec![0b1101001011111110_0010100000000000_0000000000000000_0000000000000000];
    let expected: u64 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000110;
    let actual = take_bits(&vec, 0..3).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn parse_hex_1() {
    let vec = parse_hex(TEST_1);
    assert_eq!(vec.len(), 1);
    assert_eq!(*vec.get(0).unwrap(), 15203633372514484224);
}

#[test]
fn parse_hex_2() {
    let vec = parse_hex(TEST_4);
    assert_eq!(vec.len(), 2);
    assert_eq!(*vec.get(0).unwrap(), 9944029891294921460);
    assert_eq!(*vec.get(1).unwrap(), 8646911284551352320);
}

#[allow(unused)]
pub(super) const TEST_1: &str = r#"D2FE28"#;

#[allow(unused)]
pub(super) const TEST_2: &str = r#"38006F45291200"#;

#[allow(unused)]
pub(super) const TEST_3: &str = r#"EE00D40C823060"#;

#[allow(unused)]
pub(super) const TEST_4: &str = r#"8A004A801A8002F478"#;

#[allow(unused)]
pub(super) const TEST_5: &str = r#"620080001611562C8802118E34"#;

#[allow(unused)]
pub(super) const TEST_6: &str = r#"C0015000016115A2E0802F182340"#;

#[allow(unused)]
pub(super) const TEST_7: &str = r#"C0015000016115A2E0802F182340"#;

pub(super) const INPUT_DATA: &str =
    "6053231004C12DC26D00526BEE728D2C013AC7795ACA756F93B524D8000AAC\
8FF80B3A7A4016F6802D35C7C94C8AC97AD81D30024C00D1003C80AD050029C00E20240580853401E98C00D50038400D401\
518C00C7003880376300290023000060D800D09B9D03E7F546930052C016000422234208CC000854778CF0EA7C9C802ACE0\
05FE4EBE1B99EA4C8A2A804D26730E25AA8B23CBDE7C855808057C9C87718DFEED9A008880391520BC280004260C44C8E46\
0086802600087C548430A4401B8C91AE3749CF9CEFF0A8C0041498F180532A9728813A012261367931FF43E9040191F002A\
539D7A9CEBFCF7B3DE36CA56BC506005EE6393A0ACAA990030B3E29348734BC200D980390960BC723007614C618DC600D42\
68AD168C0268ED2CB72E09341040181D802B285937A739ACCEFFE9F4B6D30802DC94803D80292B5389DFEB2A440081CE0FC\
E951005AD800D04BF26B32FC9AFCF8D280592D65B9CE67DCEF20C530E13B7F67F8FB140D200E6673BA45C0086262FBB084F\
5BF381918017221E402474EF86280333100622FC37844200DC6A8950650005C8273133A300465A7AEC08B00103925392575\
007E63310592EA747830052801C99C9CB215397F3ACF97CFE41C802DBD004244C67B189E3BC4584E2013C1F91B0BCD60AA1\
690060360094F6A70B7FC7D34A52CBAE011CB6A17509F8DF61F3B4ED46A683E6BD258100667EA4B1A6211006AD367D600AC\
BD61FD10CBD61FD129003D9600B4608C931D54700AA6E2932D3CBB45399A49E66E641274AE4040039B8BD2C933137F95A4A\
76CFBAE122704026E700662200D4358530D4401F8AD0722DCEC3124E92B639CC5AF413300700010D8F30FE1B80021506A33\
C3F1007A314348DC0002EC4D9CF36280213938F648925BDE134803CB9BD6BF3BFD83C0149E859EA6614A8C";
