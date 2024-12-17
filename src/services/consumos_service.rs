use mongodb::Collection;
use crate::models::consumos_model::Consumos;
use futures::stream::StreamExt;
use std::error::Error;
use mongodb::bson::DateTime;
use chrono::{TimeZone, Timelike, Utc, NaiveDate}; // Usamos chrono para manejar fechas y horas

pub async fn get_all_consumos_hour(
    collection: &Collection<Consumos>, 
    target_day: &str // Día específico en formato "YYYY-MM-DD"
) -> Result<Vec<u32>, Box<dyn Error>> {
    let target_date = NaiveDate::parse_from_str(target_day, "%Y-%m-%d")?;

    let mut cursor = collection.find(None, None).await?;
    let mut horas_suma: Vec<u32> = vec![0; 24]; // Inicializa un vector con 24 elementos (uno por cada hora)
    
    while let Some(consumo) = cursor.next().await {
        match consumo {
            Ok(consumo_data) => {
                // Verificar si el consumo está dentro del día objetivo
                if is_within_day(&consumo_data.fecha_ini, &target_date) || is_within_day(&consumo_data.fecha_fin, &target_date) {
                    let hora_ini = extract_hour(&consumo_data.fecha_ini);
                    let hora_fin = extract_hour(&consumo_data.fecha_fin);

                    let duracion = calculate_duration(&consumo_data.fecha_ini, &consumo_data.fecha_fin);

                    // Sumar la duración al total correspondiente a las horas
                    for hora in hora_ini..=hora_fin {
                        if hora < 24 {
                            horas_suma[hora as usize] += duracion;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error al obtener consumo: {}", e),
        }
    }
    
    Ok(horas_suma)
}

// Función para verificar si una fecha está dentro del día objetivo
fn is_within_day(fecha: &DateTime, target_date: &NaiveDate) -> bool {
    let timestamp = fecha.timestamp_millis();
    let chrono_dt = Utc.timestamp_millis_opt(timestamp);

    if let chrono::LocalResult::Single(dt) = chrono_dt {
        return &dt.naive_utc().date() == target_date;
    }

    false
}

fn extract_hour(fecha: &DateTime) -> u32 {
    let timestamp_seconds = fecha.timestamp_millis() / 1000;
    let chrono_dt = Utc.timestamp_opt(timestamp_seconds, 0);

    match chrono_dt {
        chrono::LocalResult::Single(dt) => dt.hour(),
        _ => 0,
    }
}

fn calculate_duration(fecha_ini: &DateTime, fecha_fin: &DateTime) -> u32 {
    let start = chrono::Utc.timestamp_millis_opt(fecha_ini.timestamp_millis());
    let end = chrono::Utc.timestamp_millis_opt(fecha_fin.timestamp_millis());

    match (start, end) {
        (chrono::LocalResult::Single(start), chrono::LocalResult::Single(end)) => {
            let duration = end.signed_duration_since(start); // Aquí calculamos la duración
            duration.num_minutes() as u32
        },
        _ => 0,
    }
}