fn david_a_perez(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += pos + 1;
        } else if state.count_ones() == 14 as u32 {
            return Some(idx);
        }
    }
    return None;
}

fn main() {
    let string = std::fs::read_to_string("long").unwrap();
    let string: &'static String = Box::leak(Box::new(string));
    // let string = PROD;
    let bytes = string.as_bytes();
    david_a_perez(bytes);
}
