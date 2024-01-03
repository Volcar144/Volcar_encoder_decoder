use tauri::{AppHandle, WindowBuilder, event::Event};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use hex_literal::hex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn encrypt_data(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cbc::new(Aes256, key, iv);
    let mut output = vec![0; data.len() + cipher.block_size()];
    let mut count = 0;
    cipher.encrypt(&mut output, data, &mut count).unwrap();
    output
}

fn encrypt_file(path: &str) -> Result<(), String> {
    let key = hex!("...your_encryption_key_here...");  // Replace with your actual key
    let iv = hex!("...your_initialization_vector..."); // Replace with your IV

    let mut file = File::open(path)?;
    let mut encrypted_data = Vec::new();

    loop {
        let mut buffer = vec![0; 4096];
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let encrypted_chunk = encrypt_data(&buffer[..read], &key, &iv);
        encrypted_data.extend_from_slice(&encrypted_chunk);
    }

    let output_path = format!("{}.encrypted", path);

    // Check if the output file already exists
    if Path::new(&output_path).exists() {
        // Prompt the user for confirmation before overwriting
        let overwrite = tauri::dialog::MessageBox::new(
            tauri::dialog::MessageBoxType::Warning,
            "File Exists",
            &format!("The file '{}' already exists. Overwrite?", output_path)
        ).show().unwrap();

        if !overwrite {
            return Err("Encryption aborted: User declined to overwrite existing file.");
        }
    }

    // Write encrypted data, handling errors
    match std::fs::write(&output_path, &encrypted_data) {
        Ok(_) => {
            // Notify the frontend of success
            tauri::event::emit("encryption_success", &output_path);
            Ok(())
        },
        Err(err) => Err(format!("Failed to write encrypted file: {}", err)),
    }
}