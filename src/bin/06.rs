fn main() {
    let signal = include_str!("../inputs/06.input");
    
    let mut buffer = Vec::new();
    for (i, ch) in signal.chars().enumerate() {
        if buffer.contains(&ch) {
            let index = buffer.iter().position(|&r| r == ch).unwrap();
            buffer = buffer.split_off(index + 1);
        }
        buffer.push(ch);
        if buffer.len() >= 14 {
            println!("index {}", i + 1);
            break;
        }
    }
}
