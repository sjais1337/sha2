// Merkle-Damgard Padding
// 1, then m 0's then length as 64 bit integer
fn pad_input(input: &[u8]) -> Vec<u8> {
    let size_in_bits: u64 = (input.len() as u64) * 8;
    let mut padded: Vec<u8> = input.to_vec();

    padded.push((1 << 7) as u8);

    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    
    padded.extend_from_slice(&size_in_bits.to_be_bytes());

    padded
}

// Length of Block: 512 bits for 224/256

fn main() {
    // In general the message can be anything, right now we just have strings
    // We will have a special pre-processing step to convert a string to its 
    // binary equivalent, and then proceed
    
    let input: &str = "hello";
    let chars: &[u8] = input.as_bytes();

    let padded_input: Vec<u8> = pad_input(chars);

    println!("Padded input (Hex): ");

    for byte in padded_input {
        print!("{:02x} ", byte);
    }

    println!();
}
