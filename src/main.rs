use dioxus::prelude::*;

mod api;

fn main() {
    // Configuraciones para evitar problemas gráficos
    std::env::set_var("GDK_BACKEND", "x11");
    std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    let mut trip_data = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let mut min_price = use_signal(|| "10.0".to_string());
    let mut max_price = use_signal(|| "20.0".to_string());
    let mut price_page = use_signal(|| "1".to_string());
    let mut price_per_page = use_signal(|| "5".to_string());
    let mut price_results = use_signal(|| None::<api::apicalls::GetByPriceRangeOutput>);
    let mut price_loading = use_signal(|| false);
    let mut index_search = use_signal(|| "1".to_string());

    let mut destination = use_signal(|| "236".to_string());
    let mut dest_page = use_signal(|| "1".to_string());
    let mut dest_per_page = use_signal(|| "5".to_string());
    let mut dest_results = use_signal(|| None::<api::apicalls::GetByPriceRangeOutput>);
    let mut dest_loading = use_signal(|| false);

    let fetch_trip = move |_| {
        loading.set(true);

        spawn(async move {
            match api::apicalls::get_by_index(index_search.peek().to_string()).await {
                Ok(trip) => {
                    trip_data.set(Some(format!(
                        "Viaje encontrado: Origen {} → Destino {}, Distancia: {}, Importe: ${} USD",
                        trip.pu_location_id,
                        trip.do_location_id,
                        trip.trip_distance,
                        trip.fare_amount,
                    )));
                    loading.set(false);
                }
                Err(e) => {
                    trip_data.set(Some(format!("Error: {}", e)));
                    loading.set(false);
                }
            }
        });
    };

    let mut fetch_by_price = move |_| {
        price_loading.set(true);

        let input = api::apicalls::GetByPriceRangeInput {
            min: min_price.peek().to_string(),
            max: max_price.peek().to_string(),
            page: price_page.peek().to_string(),
            per_page: price_per_page.peek().to_string(),
        };

        spawn(async move {
            match api::apicalls::get_by_price_range(&input).await {
                Ok(result) => {
                    price_results.set(Some(result));
                    price_loading.set(false);
                }
                Err(e) => {
                    price_results.set(None);
                    price_loading.set(false);
                    trip_data.set(Some(format!("Error en búsqueda por precio: {}", e)));
                }
            }
        });
    };

    let mut fetch_by_destination = move |_| {
        dest_loading.set(true);

        let input = api::apicalls::GetByDestinationInput {
            destination: destination.peek().to_string(),
            page: dest_page.peek().to_string(),
            per_page: dest_per_page.peek().to_string(),
        };

        spawn(async move {
            match api::apicalls::get_by_destination(&input).await {
                Ok(result) => {
                    dest_results.set(Some(result));
                    dest_loading.set(false);
                }
                Err(e) => {
                    dest_results.set(None);
                    dest_loading.set(false);
                    trip_data.set(Some(format!("Error en búsqueda por destino: {}", e)));
                }
            }
        });
    };

    let prev_price_page = move |_| {
        let current = price_page().parse::<i32>().unwrap_or(1);
        if current > 1 {
            price_page.set((current - 1).to_string());
            fetch_by_price(());
        }
    };

    let next_price_page = move |_| {
        if let Some(result) = price_results() {
            let current = price_page().parse::<i32>().unwrap_or(1);
            if current < result.pages as i32 {
                price_page.set((current + 1).to_string());
                fetch_by_price(());
            }
        }
    };

    let prev_dest_page = move |_| {
        let current = dest_page().parse::<i32>().unwrap_or(1);
        if current > 1 {
            dest_page.set((current - 1).to_string());
            fetch_by_destination(());
        }
    };

    let next_dest_page = move |_| {
        if let Some(result) = dest_results() {
            let current = dest_page().parse::<i32>().unwrap_or(1);
            if current < result.pages as i32 {
                dest_page.set((current + 1).to_string());
                fetch_by_destination(());
            }
        }
    };

    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif; background-color: #f5f5f5; color: #333;",

            h1 {
                style: "color: #4285F4; text-align: center;",
                "Visor de Datos de Viajes"
            }

            div {
                style: "margin-top: 30px; padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                h2 { " Get by Index" }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-bottom: 15px;",

                    div {
                        label { "Index a buscar:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: index_search,
                            oninput: move |evt| index_search.set(evt.value().clone()),
                        }
                    }

                }

                button {
                    style: "padding: 10px 20px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    disabled: loading(),
                    onclick: fetch_trip,
                    {if loading() { "Cargando..." } else { "Obtener viaje" }}
                }

                {trip_data().map(|data| {
                    rsx! {
                        div {
                            style: "margin-top: 15px; padding: 10px; border-radius: 4px; background-color: #f0f0f0;",
                            "{data}"
                        }
                    }
                })}
            }

            div {
                style: "margin-top: 30px; padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                h2 { "Get by Price Range" }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-bottom: 15px;",

                    div {
                        label { "Precio mínimo:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: min_price,
                            oninput: move |evt| min_price.set(evt.value().clone()),
                        }
                    }

                    div {
                        label { "Precio máximo:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: max_price,
                            oninput: move |evt| max_price.set(evt.value().clone()),
                        }
                    }

                    div {
                        label { "Página:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: price_page,
                            oninput: move |evt| price_page.set(evt.value().clone()),
                        }
                    }

                    div {
                        label { "Resultados por página:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: price_per_page,
                            oninput: move |evt| price_per_page.set(evt.value().clone()),
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 10px; margin-bottom: 15px;",

                    button {
                        style: "padding: 10px 20px; background-color: #FF9800; color: white; border: none; border-radius: 4px; cursor: pointer; flex: 1;",
                        disabled: price_loading(),
                        onclick: move |_| fetch_by_price(()),
                        {if price_loading() { "Cargando..." } else { "Buscar por rango de precio" }}
                    }

                    button {
                        style: "padding: 10px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        disabled: price_loading() || price_page() == "1",
                        onclick: prev_price_page,
                        "<<"
                    }

                    button {
                        style: "padding: 10px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        disabled: price_loading() || price_results().map_or(true, |r| r.page >= r.pages),
                        onclick: next_price_page,
                        ">>"
                    }
                }

                {price_results().map(|result| {
                    rsx! {
                        div {
                            style: "margin-top: 15px;",

                            div {
                                style: "padding: 10px; border-radius: 4px; background-color: #f0f0f0; margin-bottom: 10px;",
                                p { "Total: {result.total} viajes | Página {result.page} de {result.pages} | Tiempo: {result.time_ms}ms" }
                            }

                            table {
                                style: "width: 100%; border-collapse: collapse;",

                                thead {
                                    tr {
                                        style: "background-color: #f2f2f2;",
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Índice" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Origen" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Destino" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Distancia" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Importe" }
                                    }
                                }

                                tbody {
                                    {result.items.iter().map(|trip| {
                                        rsx! {
                                            tr {
                                                key: "{trip.index}",
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.index}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.pu_location_id}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.do_location_id}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.trip_distance}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "${trip.total_amount}" }
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }
                })}
            }

            div {
                style: "margin-top: 30px; padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                h2 { " Get by Destination" }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 10px; margin-bottom: 15px;",

                    div {
                        label { "ID de Destino:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: destination,
                            oninput: move |evt| destination.set(evt.value().clone()),
                        }
                    }

                    div {
                        label { "Página:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: dest_page,
                            oninput: move |evt| dest_page.set(evt.value().clone()),
                        }
                    }

                    div {
                        label { "Resultados por página:" }
                        input {
                            style: "width: 90%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                            value: dest_per_page,
                            oninput: move |evt| dest_per_page.set(evt.value().clone()),
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 10px; margin-bottom: 15px;",

                    button {
                        style: "padding: 10px 20px; background-color: #673AB7; color: white; border: none; border-radius: 4px; cursor: pointer; flex: 1;",
                        disabled: dest_loading(),
                        onclick: move |_| fetch_by_destination(()),
                        {if dest_loading() { "Cargando..." } else { "Buscar por destino" }}
                    }

                    button {
                        style: "padding: 10px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        disabled: dest_loading() || dest_page() == "1",
                        onclick: prev_dest_page,
                        "<<"
                    }

                    button {
                        style: "padding: 10px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        disabled: dest_loading() || dest_results().map_or(true, |r| r.page >= r.pages),
                        onclick: next_dest_page,
                        ">>"
                    }
                }

                {dest_results().map(|result| {
                    rsx! {
                        div {
                            style: "margin-top: 15px;",

                            div {
                                style: "padding: 10px; border-radius: 4px; background-color: #f0f0f0; margin-bottom: 10px;",
                                p { "Total: {result.total} viajes | Página {result.page} de {result.pages} | Tiempo: {result.time_ms}ms" }
                            }

                            table {
                                style: "width: 100%; border-collapse: collapse;",

                                thead {
                                    tr {
                                        style: "background-color: #f2f2f2;",
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Índice" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Origen" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Destino" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Distancia" }
                                        th { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "Importe" }
                                    }
                                }

                                tbody {
                                    {result.items.iter().map(|trip| {
                                        rsx! {
                                            tr {
                                                key: "{trip.index}",
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.index}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.pu_location_id}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.do_location_id}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "{trip.trip_distance}" }
                                                td { style: "padding: 8px; text-align: left; border-bottom: 1px solid #ddd;", "${trip.total_amount}" }
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
