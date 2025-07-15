use crate::api::apicalls::{get_by_price_range, GetByPriceRangeInput, GetByPriceRangeOutput, Trip};
use dioxus::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[component]
pub fn PriceRangeSearch() -> Element {
    // Estado para parámetros de entrada
    let mut min_price = use_signal(|| "10".to_string());
    let mut max_price = use_signal(|| "200".to_string());
    let mut current_page = use_signal(|| 1u32); // Cambiado a u32 en lugar de String
    let mut items_per_page = use_signal(|| "10".to_string());

    // Estado para resultados y control
    let mut search_results = use_signal(|| None::<GetByPriceRangeOutput>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut debug_info = use_signal(|| "Ninguna búsqueda realizada aún".to_string());

    // Crear una función compartida para hacer búsquedas
    let fetch_data = move |page: u32, is_initial_search: bool| {
        if loading() {
            return; // Evitar múltiples solicitudes simultáneas
        }

        loading.set(true);
        error.set(None);

        // Si es búsqueda inicial, actualiza la página a 1
        if is_initial_search {
            current_page.set(1);
        } else {
            current_page.set(page);
        }

        let page_str = page.to_string();
        debug_info.set(format!("📝 Solicitando página {}", page));

        // Importante: usar el número de página directamente aquí
        let input = GetByPriceRangeInput {
            min: min_price(),
            max: max_price(),
            pages: page_str.clone(), // ¡Prueba con 'pages'!
            per_page: items_per_page(),
        };

        // Registra para depuración exactamente qué parámetros se envían
        debug_info.set(format!(
            "🔍 Enviando: min={}, max={}, pages={}, per_page={}",
            input.min, input.max, input.pages, input.per_page
        ));

        // Importante: captura los valores actuales para verificación
        let expected_page = page;

        spawn(async move {
            match get_by_price_range(&input).await {
                Ok(results) => {
                    // Verificar si la API devolvió la página esperada
                    if results.page != expected_page {
                        let warning = format!(
                            "⚠️ Advertencia: Solicitamos página {} pero recibimos página {}",
                            expected_page, results.page
                        );
                        println!("{}", warning);
                        debug_info.set(format!("{}\n{}", debug_info(), warning));
                    }

                    let msg = format!(
                        "✅ Resultados página {}: {} elementos de {} totales (páginas: {})",
                        results.page,
                        results.items.len(),
                        results.total,
                        results.pages
                    );
                    println!("{}", msg);
                    debug_info.set(format!("{}\n{}", debug_info(), msg));

                    search_results.set(Some(results));
                    loading.set(false);
                }
                Err(e) => {
                    let err_msg = format!("❌ Error al obtener datos: {}", e);
                    println!("{}", err_msg);
                    debug_info.set(format!("{}\n{}", debug_info(), err_msg));
                    error.set(Some(err_msg));
                    loading.set(false);
                }
            }
        });
    };

    // Función para realizar la búsqueda inicial
    let perform_search = move |_| {
        fetch_data(1, true); // Siempre comienza en la página 1
    };

    // Navegar a otra página (simplificado)
    let go_to_page = move |new_page: u32| {
        if new_page == 0 || new_page == current_page() {
            return; // No hacer nada para elipsis o la página actual
        }

        // Añadir mensaje que indique el intento de cambio
        debug_info.set(format!(
            "🔄 Cambiando de página {} a página {}",
            current_page(),
            new_page
        ));

        // Llamar a la función compartida con la nueva página
        fetch_data(new_page, false);
    };

    // El resto del código sigue igual, con los rsx!...

    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif;",

            h2 { "Búsqueda por Rango de Precios" }

            // Panel de depuración ampliado
            div {
                style: "margin-bottom: 15px; padding: 10px; background-color: #f0f8ff; border: 1px solid #ccc; border-radius: 4px; font-family: monospace; max-height: 150px; overflow-y: auto;",
                h4 { style: "margin-top: 0;", "🔍 Panel de Depuración" }
                p { "Página actual: {current_page}" }
                pre { style: "margin: 0; white-space: pre-wrap;", "{debug_info}" }

                // Botón para forzar una recarga de la página actual (útil para pruebas)
                button {
                    style: "margin-top: 10px; padding: 5px; font-size: 12px;",
                    onclick: move |_| fetch_data(current_page(), false),
                    "🔄 Recargar página actual"
                }
            }

            // Formulario de búsqueda
            div {
                style: "background-color: #f9f9f9; padding: 15px; border-radius: 8px; margin-bottom: 20px;",

                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px;",

                    div {
                        label {
                            style: "display: block; margin-bottom: 5px; font-weight: bold;",
                            "Precio Mínimo ($)"
                        }
                        input {
                            style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: "{min_price}",
                            oninput: move |e| min_price.set(e.value().clone()),
                            placeholder: "Ej: 10"
                        }
                    }

                    div {
                        label {
                            style: "display: block; margin-bottom: 5px; font-weight: bold;",
                            "Precio Máximo ($)"
                        }
                        input {
                            style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: "{max_price}",
                            oninput: move |e| max_price.set(e.value().clone()),
                            placeholder: "Ej: 200"
                        }
                    }

                    div {
                        label {
                            style: "display: block; margin-bottom: 5px; font-weight: bold;",
                            "Items por página"
                        }
                        select {
                            style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: "{items_per_page}",
                            oninput: move |e| items_per_page.set(e.value().clone()),

                            option { value: "5", "5" }
                            option { value: "10", "10" }
                            option { value: "20", "20" }
                            option { value: "50", "50" }
                        }
                    }
                }

                div {
                    style: "margin-top: 15px; display: flex; justify-content: center;",

                    button {
                        style: "padding: 10px 20px; background-color: #4285F4; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        disabled: loading(),
                        onclick: perform_search,
                        {if loading() { "Buscando..." } else { "Buscar Viajes" }}
                    }
                }
            }

            // Mostrar error si existe
            {if let Some(err) = error() {
                rsx! {
                    div {
                        style: "color: #721c24; background-color: #f8d7da; padding: 12px; border-radius: 4px; margin-bottom: 15px; border: 1px solid #f5c6cb;",
                        "{err}"
                    }
                }
            } else { rsx!{} }}

            // Mostrar resultados
            {match (loading(), search_results()) {
                (true, _) => rsx! {
                    div {
                        style: "text-align: center; padding: 40px;",
                        "Cargando resultados..."
                    }
                },
                (false, Some(results)) => rsx! {
                    div {
                        // Información de resultados
                        div {
                            style: "margin-bottom: 15px; display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: 10px;",

                            p {
                                "Mostrando {results.items.len()} de {results.total} viajes • Página {results.page} de {results.pages}"
                            }

                            // Controles de paginación
                            {if results.pages > 1 {
                                rsx! {
                                    div {
                                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                                        button {
                                            style: "padding: 5px 10px; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer;",
                                            disabled: results.page <= 1 || loading(),
                                            onclick: move |_| {
                                                // Aquí utilizamos un valor directo
                                                let prev_page = if results.page > 1 { results.page - 1 } else { 1 };
                                                go_to_page(prev_page)
                                            },
                                            "← Anterior"
                                        }

                                        // Mostrar solo algunas páginas para no saturar la interfaz
                                        {get_pagination_buttons(results.page, results.pages).iter().map(|&page| {
                                            if page == 0 {
                                                // Elipsis - no clickeable
                                                rsx! {
                                                    span {
                                                        key: "ellipsis",
                                                        style: "padding: 5px 10px; display: flex; align-items: center;",
                                                        "..."
                                                    }
                                                }
                                            } else {
                                                // Botón de página normal - IMPORTANTE: Capturamos el valor específico
                                                let is_current = page == results.page;
                                                let page_num = page; // Capturar valor
                                                rsx! {
                                                    button {
                                                        key: "{page}",
                                                        style: if is_current {
                                                            "padding: 5px 10px; border: 1px solid #ddd; border-radius: 4px; background-color: #4285F4; color: white; cursor: pointer;"
                                                        } else {
                                                            "padding: 5px 10px; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer;"
                                                        },
                                                        onclick: move |_| {
                                                            // Usamos el valor capturado en lugar de la referencia
                                                            go_to_page(page_num)
                                                        },
                                                        "{page}"
                                                    }
                                                }
                                            }
                                        })}

                                        button {
                                            style: "padding: 5px 10px; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer;",
                                            disabled: results.page >= results.pages || loading(),
                                            onclick: move |_| {
                                                // Aquí también usamos un valor directo
                                                let next_page = if results.page < results.pages { results.page + 1 } else { results.pages };
                                                go_to_page(next_page)
                                            },
                                            "Siguiente →"
                                        }
                                    }
                                }
                            } else { rsx!{} }}
                        }

                        // Tabla de resultados
                        {if results.items.is_empty() {
                            rsx! {
                                div {
                                    style: "text-align: center; padding: 30px; background-color: #f9f9f9; border-radius: 4px;",
                                    "No se encontraron viajes en este rango de precios."
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "overflow-x: auto;",
                                    table {
                                        style: "width: 100%; border-collapse: collapse;",

                                        thead {
                                            tr {
                                                style: "background-color: #f0f0f0;",
                                                th { style: "text-align: left; padding: 12px 8px; border-bottom: 2px solid #ddd;", "ID" }
                                                th { style: "text-align: left; padding: 12px 8px; border-bottom: 2px solid #ddd;", "Fecha" }
                                                th { style: "text-align: left; padding: 12px 8px; border-bottom: 2px solid #ddd;", "Origen-Destino" }
                                                th { style: "text-align: left; padding: 12px 8px; border-bottom: 2px solid #ddd;", "Distancia" }
                                                th { style: "text-align: right; padding: 12px 8px; border-bottom: 2px solid #ddd;", "Tarifa" }
                                                th { style: "text-align: right; padding: 12px 8px; border-bottom: 2px solid #ddd;", "Total" }
                                            }
                                        }

                                        tbody {
                                            // Mostrar cada viaje
                                            {results.items.iter().map(|trip| {
                                                rsx! {
                                                    tr {
                                                        key: "{trip.index}",
                                                        style: "border-bottom: 1px solid #eee;",

                                                        td { style: "padding: 12px 8px;", "{trip.index}" }
                                                        td {
                                                            style: "padding: 12px 8px;",
                                                            "{format_date(&trip.tpep_pickup_datetime)}"
                                                        }
                                                        td {
                                                            style: "padding: 12px 8px;",
                                                            "#{trip.pu_location_id} → #{trip.do_location_id}"
                                                        }
                                                        td { style: "padding: 12px 8px;", "{trip.trip_distance} mi" }
                                                        td {
                                                            style: "padding: 12px 8px; text-align: right;",
                                                            "${format_currency(&trip.fare_amount)}"
                                                        }
                                                        td {
                                                            style: "padding: 12px 8px; text-align: right; font-weight: bold;",
                                                            "${format_currency(&trip.total_amount)}"
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }
                        }}

                        // Estadísticas simples
                        {if !results.items.is_empty() {
                            let total_fare: f64 = results.items.iter()
                                .filter_map(|trip| trip.fare_amount.parse::<f64>().ok())
                                .sum();

                            let avg_fare = if results.items.len() > 0 {
                                total_fare / results.items.len() as f64
                            } else {
                                0.0
                            };

                            rsx! {
                                div {
                                    style: "margin-top: 20px; padding: 15px; background-color: #f5f5f5; border-radius: 4px;",
                                    p {
                                        style: "margin: 0;",
                                        "Tarifa promedio: "
                                        span { style: "font-weight: bold;", "${avg_fare:.2}" }
                                    }
                                }
                            }
                        } else { rsx!{} }}
                    }
                },
                (false, None) => rsx! {
                    div {
                        style: "text-align: center; color: #666; padding: 40px;",
                        "Ingresa un rango de precios y presiona 'Buscar Viajes'"
                    }
                }
            }}
        }
    }
}

// Función auxiliar para formatear fechas
fn format_date(date_str: &str) -> String {
    // Extraer solo la fecha de la cadena ISO
    if date_str.len() >= 10 {
        return date_str[..10].to_string();
    }
    date_str.to_string()
}

// Función auxiliar para formatear moneda
fn format_currency(amount_str: &str) -> String {
    match amount_str.parse::<f64>() {
        Ok(amount) => format!("{:.2}", amount),
        Err(_) => amount_str.to_string(),
    }
}

// Función para generar botones de paginación
fn get_pagination_buttons(current: u32, total: u32) -> Vec<u32> {
    let mut buttons = Vec::new();

    // Siempre mostrar la primera página
    buttons.push(1);

    // Calcular rango de páginas alrededor de la página actual
    let start = (current as i32 - 2).max(2) as u32;
    let end = (current as i32 + 2).min(total as i32).max(start as i32) as u32;

    // Si hay un salto entre 1 y start, añadir elipsis (representado como 0)
    if start > 2 {
        buttons.push(0); // 0 representa "..."
    }

    // Añadir páginas del rango
    for i in start..=end {
        buttons.push(i);
    }

    // Si hay un salto entre end y total, añadir elipsis
    if end < total - 1 {
        buttons.push(0); // 0 representa "..."
    }

    // Siempre mostrar la última página si hay más de una página
    if total > 1 && !buttons.contains(&total) {
        buttons.push(total);
    }

    buttons
}
