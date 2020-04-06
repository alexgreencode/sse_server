use jwt::{decode, Validation, Algorithm, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub fn check(sub_opt: Option<String>, token: &str) -> Result<(), &'static str> {
    let sub = match sub_opt {
        Some(value) => {
            Some(value.to_lowercase())
        },
        None => None
    };

    let validation = Validation {
        sub,
        leeway: 60,
        ..Validation::new(Algorithm::RS256)
    };

    match DecodingKey::from_rsa_pem(include_bytes!("../cert/public_key.pem")) {
        Ok(key) => {
            if let Err(e) = decode::<Claims>(token, &key, &validation) {
                println!("Invalid token -> {}", e);
                return Err("Invalid token");
            };
            Ok(())
        },
        Err(e) => {
            println!("get_token error - loading key -> {}", e);
            Err("error loading key ")
        }
    }
}
