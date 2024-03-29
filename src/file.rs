use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

use crate::structs::auth::AuthResponse;
use crate::utils::custom_exit;

pub fn store_token_in_tmp(auth_response: &AuthResponse) {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + "/ceosvm.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(_) => {
            custom_exit(Some(
                "Impossible d'ouvrir le fichier. Vérifiez le chemin et les autorisations."
                    .to_string(),
            ));
            return;
        }
    };

    if let Ok(metadata) = file.metadata() {
        let mut bytes: Vec<u8> = Vec::new();
        serde_json::to_writer(&mut bytes, &auth_response).unwrap();

        if metadata.len() == 0 {
            if let Err(_) = file.write_all(&bytes) {
                custom_exit(Some(
                    "Erreur lors de l'écriture dans le fichier.".to_string(),
                ));
            }
        }
    }
}

pub fn read_tmp_file() -> Result<AuthResponse, bool> {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + "/ceosvm.json";

    let file = File::open(&path);
    let file = match file {
        Ok(file) => file,
        Err(_) => return Err(true),
    };
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader);
    match data {
        Ok(data) => Ok(data),
        Err(_) => Err(true),
    }
}

pub fn remove_tmp_file() {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + "/ceosvm.json";

    let _ = std::fs::remove_file(path);
}
