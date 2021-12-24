pub fn hexdump(byte_array: &[u8]) {
    let mut offset = 0;
    while offset < byte_array.len() {
        let mut length = 16;
        if byte_array.len() - offset < 16 {
            length = byte_array.len() - offset;
        }
        println!("{:08x}: {:49} {:16}",
                 offset,
                 get_hex_representation(&byte_array[offset..offset+length]),
                 get_ascii_representation(&byte_array[offset..offset+length]));
        offset += 16;
    }
}

fn get_hex_representation(byte_array: &[u8]) -> String {
    byte_array
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if i == 7 {
                format!("{:02x} ", val)
            } else {
                format!("{:02x}", val)
            }
        }).collect::<Vec<String>>()
        .join(" ")
}

fn get_ascii_representation(byte_array: &[u8]) -> String {
    byte_array
        .iter()
        .map(|num| {
            match *num {
                32..=126 => (*num as char).to_string(),
                _ => '.'.to_string()
            }
        }).collect::<Vec<String>>()
        .join("")
}
