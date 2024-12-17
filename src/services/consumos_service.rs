use mongodb::Collection;
use crate::models::consumos_model::Consumos;
use futures::stream::StreamExt;
use std::error::Error;
use mongodb::bson::DateTime;
use chrono::{Datelike, NaiveDate, TimeZone, Timelike, Utc}; // Usamos chrono para manejar fechas y horas

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

                    let duracion = calculate_duration_days(&consumo_data.fecha_ini, &consumo_data.fecha_fin);

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

fn calculate_duration_days(fecha_ini: &DateTime, fecha_fin: &DateTime) -> u32 {
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

//Meses

pub async fn get_all_consumos_month(
    collection: &Collection<Consumos>,
) -> Result<Vec<u32>, Box<dyn Error>> {
    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month();
    let num_days = days_in_month(current_year, current_month);

    let mut cursor = collection.find(None, None).await?;
    let mut dias_suma: Vec<u32> = vec![0; num_days as usize]; // Inicializa un vector con un elemento por día del mes

    while let Some(consumo) = cursor.next().await {
        match consumo {
            Ok(consumo_data) => {
                let fecha_ini = consumo_data.fecha_ini;
                let fecha_fin = consumo_data.fecha_fin;

                if is_within_month(&fecha_ini, current_year, current_month)
                    || is_within_month(&fecha_fin, current_year, current_month)
                {
                    let day_ini = extract_day(&fecha_ini);
                    let day_fin = extract_day(&fecha_fin);

                    let duracion = calculate_duration_months(&fecha_ini, &fecha_fin);

                    for day in day_ini..=day_fin {
                        if day >= 1 && day <= num_days {
                            dias_suma[(day - 1) as usize] += duracion;
                        } else {
                            eprintln!("Error: Día fuera de rango: {}", day);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error al obtener consumo: {}", e),
        }
    }

    Ok(dias_suma)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }

        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn is_within_month(fecha: &DateTime, year: i32, month: u32) -> bool {
    let timestamp = fecha.timestamp_millis();
    let chrono_dt = Utc.timestamp_millis_opt(timestamp);

    if let chrono::LocalResult::Single(dt) = chrono_dt {
        return dt.year() == year && dt.month() == month;
    }

    false
}

fn extract_day(fecha: &DateTime) -> u32 {
    let timestamp_seconds = fecha.timestamp_millis() / 1000;
    let chrono_dt = Utc.timestamp_opt(timestamp_seconds, 0);

    match chrono_dt {
        chrono::LocalResult::Single(dt) => dt.day(),
        _ => 0,
    }
}

fn calculate_duration_months(fecha_ini: &DateTime, fecha_fin: &DateTime) -> u32 {
    let start = chrono::Utc.timestamp_millis_opt(fecha_ini.timestamp_millis());
    let end = chrono::Utc.timestamp_millis_opt(fecha_fin.timestamp_millis());

    match (start, end) {
        (chrono::LocalResult::Single(start), chrono::LocalResult::Single(end)) => {
            let duration = end.signed_duration_since(start);
            duration.num_minutes() as u32
        }
        _ => 0,
    }
}

pub async fn get_all_consumos_year(
    collection: &Collection<Consumos>,
) -> Result<Vec<u32>, Box<dyn Error>> {
    let now = Utc::now();
    let current_year = now.year();

    let mut cursor = collection.find(None, None).await?;
    let mut meses_suma: Vec<u32> = vec![0; 12]; // Inicializa un vector con un elemento por cada mes del año

    while let Some(consumo) = cursor.next().await {
        match consumo {
            Ok(consumo_data) => {
                let fecha_ini = consumo_data.fecha_ini;
                let fecha_fin = consumo_data.fecha_fin;

                // Verificar si el consumo está dentro del año actual
                if is_within_year(&fecha_ini, current_year) || is_within_year(&fecha_fin, current_year) {
                    // Obtener el mes inicial y final del consumo
                    let mes_ini = extract_month(&fecha_ini);
                    let mes_fin = extract_month(&fecha_fin);

                    let duracion = calculate_duration_year(&fecha_ini, &fecha_fin);

                    // Sumar la duración al total correspondiente a los meses
                    for mes in mes_ini..=mes_fin {
                        if mes >= 1 && mes <= 12 {
                            meses_suma[(mes - 1) as usize] += duracion; // Mes 1 se mapea al índice 0
                        } else {
                            eprintln!("Error: Mes fuera de rango: {}", mes);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error al obtener consumo: {}", e),
        }
    }

    Ok(meses_suma)
}

fn is_within_year(fecha: &DateTime, year: i32) -> bool {
    let timestamp = fecha.timestamp_millis();
    let chrono_dt = Utc.timestamp_millis_opt(timestamp);

    if let chrono::LocalResult::Single(dt) = chrono_dt {
        return dt.year() == year;
    }

    false
}

fn extract_month(fecha: &DateTime) -> u32 {
    let timestamp_seconds = fecha.timestamp_millis() / 1000;
    let chrono_dt = Utc.timestamp_opt(timestamp_seconds, 0);

    match chrono_dt {
        chrono::LocalResult::Single(dt) => dt.month(),
        _ => 0,
    }
}

fn calculate_duration_year(fecha_ini: &DateTime, fecha_fin: &DateTime) -> u32 {
    let start = chrono::Utc.timestamp_millis_opt(fecha_ini.timestamp_millis());
    let end = chrono::Utc.timestamp_millis_opt(fecha_fin.timestamp_millis());

    match (start, end) {
        (chrono::LocalResult::Single(start), chrono::LocalResult::Single(end)) => {
            let duration = end.signed_duration_since(start); // Calcula la duración
            duration.num_minutes() as u32
        }
        _ => 0,
    }
}
