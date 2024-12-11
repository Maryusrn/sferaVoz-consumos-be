use bcrypt::verify;
use mongodb::Collection;
use mongodb::bson::doc;
use crate::models::user_model::User;
use crate::services::jwt_service::generate_token;
use log::info;

pub async fn login_service(
    email: &str,
    password: &str,
    user: &Collection<User>,
    jwt_secret: &str,
) -> Result<String, String> {
    info!("Buscando usuario con el correo: {}", email);

    match user
        .find_one(doc! { "email": email }, None)
        .await
    {
        Ok(Some(user)) => {            
            match verify(password, &user.password) {
                Ok(true) => {
                    info!("Contraseña verificada correctamente");

                    // Genera el token JWT
                    match generate_token(
                        &user,
                        jwt_secret,
                    ) {
                        Ok(token) => Ok(token),
                        Err(err) => {
                            info!("Error al generar el token JWT: {}", err);
                            Err("Error al generar el token JWT".to_string())
                        }
                    }
                }
                Ok(false) => {
                    info!("Contraseña incorrecta");
                    Err("Contraseña incorrecta".to_string())
                }
                Err(err) => {
                    info!("Error al verificar la contraseña: {}", err);
                    Err("Error al verificar la contraseña".to_string())
                }
            }
        }
        Ok(None) => {
            info!("No se encontró usuario con el correo: {}", email);
            Err("Usuario no encontrado".to_string())
        }
        Err(err) => {
            info!("Error al buscar el usuario: {}", err);
            Err(format!("Error al buscar el usuario: {}", err))
        }
    }
}