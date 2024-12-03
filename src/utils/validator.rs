use regex::Regex;

pub fn validate_email(email: &str) -> Result<(), String> {

    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(String::from("El formato del email es inválido"))
    }
}

pub fn validate_password(password: &str) -> Result<(), String> {

    let password_regex = Regex::new(r"([a-z].*[A-Z].*\d|\d.*[a-z].*[A-Z]|[A-Z].*[a-z].*\d|[A-Z].*\d.*[a-z]|[a-z].*\d.*[A-Z]|[a-zA-Z].*\d.{8,})").unwrap();

    if password_regex.is_match(password) {
        Ok(())
    } else {
        Err(String::from("La contraseña debe tener al menos 8 caracteres, incluyendo mayúsculas, minúsculas y números"))
    }
}