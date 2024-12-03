use bcrypt::verify;
use mongodb::Collection;
use mongodb::bson::doc;
use crate::models::user_model::User;
use log::info;

pub async fn login_service(
    email: &str,
    password: &str,
    user_collection: &Collection<User>,
) -> Result<bool, String> {
    info!("Buscando usuario con el correo: {}", email);

    match user_collection
        .find_one(doc! { "email": email }, None)
        .await
    {
        Ok(Some(user)) => {            
            match verify(password, &user.password) {
                Ok(true) => {
                    info!("Contraseña verificada correctamente");
                    Ok(true)
                }
                Ok(false) => {
                    info!("Contraseña incorrecta");
                    Err("Contraseña incorrecta".to_string())
                }
                Err(err) => {
                    info!("Error al verificar la passwprd: {}", err);
                    Err("Error al verificar la password".to_string())
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