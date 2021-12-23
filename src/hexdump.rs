pub fn hexdump(byte_array: &[u8]) {
    let mut offset = 0;
    while offset < byte_array.len() {
        let mut length = 16;
        if byte_array.len() - offset < 16 {
            length = byte_array.len() - offset;
        }
        println!("0x{:08x}: {:49} {:16}",
                 offset,
                 get_hex_representation(&byte_array[offset..offset+length]),
                 get_ascii_representation(&byte_array[offset..offset+length]));
        offset += 16;
    }
}

fn get_hex_representation(byte_array: &[u8]) -> String {
    let build_string_vec: Vec<String> = byte_array.iter().enumerate()
        .map(|(i, val)| {
            if i == 7 {
                format!("{:02x} ", val)
            } else {
                format!("{:02x}", val)
            }
        }).collect();
    build_string_vec.join(" ")
}

fn get_ascii_representation(byte_array: &[u8]) -> String {
    let build_string_vec: Vec<String> = byte_array.iter().map(|num| {
        if 32 <= *num && *num <= 126 {
            (*num as char).to_string()
        } else {
            '.'.to_string()
        }
    }).collect();
    build_string_vec.join("")
}
