// Merkle-Damgard Padding
// 1, then m 0's then length as 64 bit integer
const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;

const K: [u32; 64] = [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
                    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
                    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
                    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
                    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
                    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
                    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
                    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];

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

fn ch(x: u32, y: u32, z: u32) -> u32 {
    let output: u32 = (x & y) ^ ((!x) & z); 
    output
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    let output: u32 = (x & y) ^ (y & z) ^ (x & z);
    output
}

fn cal_s0(x: u32) -> u32 {
    let output: u32 = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3); 
    output
}

fn cal_s1(x: u32) -> u32 {
    let output: u32 = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10); 
    output
}

fn cal_sigma0(x: u32) -> u32 {
    let output: u32 = x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22);
    output
}

fn cal_sigma1(x: u32) -> u32 {
    let output: u32 = x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25);
    output
}
// Length of Block: 512 bits for 224/256

fn main() {
    // In general the message can be anything, right now we just have strings
    // We will have a special pre-processing step to convert a string to its 
    // binary equivalent, and then proceed
    
    let input: &str = "hello";
    let chars: &[u8] = input.as_bytes();

    let padded_input: Vec<u8> = pad_input(chars);
    let (split_32,_) =  padded_input.as_chunks::<4>();

    let padded_words: Vec<u32> = split_32.iter().map(|&x| u32::from_be_bytes(x)).collect();

    let mut h0: u32 = H0;
    let mut h1: u32 = H1;
    let mut h2: u32 = H2;
    let mut h3: u32 = H3;
    let mut h4: u32 = H4;
    let mut h5: u32 = H5;
    let mut h6: u32 = H6;
    let mut h7: u32 = H7;

    let (chunks, _) = padded_words.as_chunks::<16>();

    for chunk in chunks.iter() { 
        let mut message_schedule: [u32; 64] = [0; 64];
        
        message_schedule[0..16].copy_from_slice(chunk);

        for i in 16..64 {
            let s0 = cal_s0(message_schedule[i-15]);
            let s1 = cal_s1(message_schedule[i-2]);

            message_schedule[i] = message_schedule[i-16]
                .wrapping_add(s0)
                .wrapping_add(s1)
                .wrapping_add(message_schedule[i - 7]);
        }

        let mut a: u32 = h0;
        let mut b: u32 = h1;
        let mut c: u32 = h2;
        let mut d: u32 = h3;
        let mut e: u32 = h4;
        let mut f: u32 = h5;
        let mut g: u32 = h6;
        let mut h: u32 = h7;

        for i in 0..64 {
            let temp1: u32 = h
            .wrapping_add(cal_sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(message_schedule[i]);

            let temp2: u32 = cal_sigma0(a).wrapping_add(maj(a, b, c)); 

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c; 
            c = b; 
            b = a; 
            a = temp1.wrapping_add(temp2);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }

    println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4, h5, h6, h7);
}
