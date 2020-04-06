use std::env;

pub fn get_query_from_request(query: &str, key: &str) -> String {
    return match query.split("&").find(|item| {
        item.starts_with(format!("{}=", key).as_str())
    }) {
        Some(sub) => sub.replace(&format!("{}=", key), ""),
        None => "".to_string()
    };
}

pub fn get_sub_from_query(query: &str) -> String {
    return get_query_from_request(query, "sub");
}

pub fn get_token_from_query(query: &str) -> String {
    return get_query_from_request(query, "token");
}

pub fn get_auth_header() -> String {
    let token = env::var("AUTH_TOKEN_TEST_API").unwrap_or("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiI4Njk4NmZjNi0wOTI0LTExZWEtYjdhYS0wMjQyYWMxMzAwMDIiLCJleHAiOjE1NzkzNDkyNDh9.NvyxFFLijcanZKQf7fFoS-xlzy-dpvfuOoDTbFgQWWgt-jasgLXtNnnU2QMGWHwhAiir001QWrHyEkawEb4-Vxk9AiDY9axlYQRmeB0GLWjD1XknFVKr7hCGQ9WAlaeIVcalOEVlwiP97sM6Hju8dd95yML2c6JrxDc1vML9v7Cw3olfhKt6CqZ0e3DuDlTq9V3gE3G6F52-stwoAkkxfh3ngLLfR9hC1m773etaCHQiJtyWRiSlzvvnal4B3zzr1KK4OwcEals61ScfwDAbI4CdlxTKWiYXSSQOXqcra6L-9MN94P2vxewFFlJRKLmUACtkTNJVbDeYXXhhIsVvUQ".to_string());

    format!("Bearer {}", token)
}

pub fn get_auth_sub() -> String {
    let sub = env::var("AUTH_SUB_TEST_API").unwrap_or("86986fc6-0924-11ea-b7aa-0242ac130002".to_string());

    format!("sub={}", sub)
}
