pub mod  encrypt;
pub mod path_traversal;
pub mod permission_check;

mod tests {
    #![allow(unused_imports)]
    use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit};

    #[allow(dead_code)]
    struct Node {
        child:Vec<Node>,
        name:String,
        is_file:bool
    }
    /* 
    N (root)
    / | \
    (file.txt) N  N  N (etc, dev)
    |
    N (test.conf)
    */
    #[test]
    fn directory_discovery() -> Result<(), std::io::Error> {
        use std::collections::VecDeque;
        let root = Node {name: "Root".to_string(), is_file: false, child: vec![
            Node{name: "file.txt".to_string(), is_file: true, child: vec![]},
            Node{name: "etc".to_string(), is_file: false, child: vec![]},
            Node{name: "dev".to_string(), is_file: false, child: vec![
                Node{name: "test.conf".to_string(), is_file: true, child: vec![]},
            ]}
        ]};
        let mut stack:VecDeque<Node> = VecDeque::from(root.child);
        while stack.len() != 0 {
            let current_node = stack.pop_front().unwrap();
            stack.extend(current_node.child);
            println!("{}", current_node.name);
        }
        Ok(())
    }

    #[test]
    fn encrpytion() -> Result<(), aes_gcm::Error> {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
        assert_eq!(&plaintext, b"plaintext message");
        Ok(())
    }
    
    #[test]
    fn file_encryption() -> Result<(), aes_gcm::Error> {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        // 46 bytes of text
        // whole content of a file
        let text = String::from("I am a file that happens to be encrypted soon!");
        let converted_text = text.as_bytes();
        let mut encrypted_text:Vec<Vec<u8>> = vec![];
        
        // encrypting the text by splitting in 2
        // mimics a buffered reading
        for chunk in converted_text.chunks(23) {
            let encrypted_chunk = cipher.encrypt(&nonce, chunk.as_ref())?;
            encrypted_text.push(encrypted_chunk);
        }
    
        // decrypting the text
        let mut decrypted_text = String::new();
        for chunk in &encrypted_text {
            let decrypted_chunk = cipher.decrypt(&nonce, chunk.as_ref())?;
            let text_chunk = String::from_utf8(decrypted_chunk).unwrap();
            decrypted_text.push_str(
                &text_chunk
            );
        }
        assert_eq!("I am a file that happens to be encrypted soon!", decrypted_text);
        Ok(())
    }
    
    #[test]
    fn encrypting_multiple_text() -> Result<(), std::io::Error> {
        let text1 = String::from("I am text 1");
        let text2 = String::from("I am test 2");
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce1 = Aes256Gcm::generate_nonce(&mut OsRng);
        let nonce2 = Aes256Gcm::generate_nonce(&mut OsRng);
    
        let encrypted_text1 = cipher.encrypt(&nonce1, text1.as_ref()).unwrap();
        let encrypted_text2 = cipher.encrypt(&nonce2, text2.as_ref()).unwrap();
    
        assert_eq!(cipher.decrypt(&nonce1, encrypted_text1.as_ref()).unwrap(), text1.as_bytes().to_vec());
        assert_eq!(cipher.decrypt(&nonce2, encrypted_text2.as_ref()).unwrap(), text2.as_bytes().to_vec());
        Ok(())
    }
}
