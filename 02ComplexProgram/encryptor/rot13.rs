pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        let mut new_string = String::new();
	for ch in self.0.chars() {
            if (ch >= 'a' && ch < 'n') || (ch >= 'A' && ch < 'N') {
               new_string.push((ch as u8 + 13) as char); 
            } else if (ch >= 'n' && ch < 'z') || (ch >= 'N' && ch < 'Z') {
               new_string.push((ch as u8 - 13) as char);
            } else {
               new_string.push(ch);
            }
        }
        new_string
    }
}
