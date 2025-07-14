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
    // Declarar count como mutable
    let mut count = use_signal(|| 0);

    // Estado para almacenar resultado de la API
    let mut trip_data = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    // Función para realizar la llamada a la API
    let fetch_trip = move |_| {
        loading.set(true);

        // Ejecutar código asíncrono con spawn
        spawn(async move {
            // Convertir el índice a String y realizar la llamada
            match api::apicalls::get_by_index("1".to_string()).await {
                Ok(trip) => {
                    // Convertir el resultado a una cadena para mostrar
                    trip_data.set(Some(format!(
                        "Viaje encontrado: Origen {} → Destino {}, Distancia: {}",
                        trip.pu_location_id, trip.do_location_id, trip.trip_distance
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

    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif; background-color: #f5f5f5; color: #333;",

            h1 {
                style: "color: #4285F4; text-align: center;",
                "Visor de Datos de Viajes"
            }

            p {
                style: "text-align: center; margin: 20px 0;",
                "Contador: {count}"
            }

            div {
                style: "display: flex; justify-content: center; gap: 10px; margin-top: 30px;",

                button {
                    style: "padding: 10px 20px; background-color: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| count.set(count() + 1),
                    "Incrementar"
                }

                button {
                    style: "padding: 10px 20px; background-color: #F44336; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| count.set(0),
                    "Resetear"
                }
            }

            // Sección para probar la API
            div {
                style: "margin-top: 30px; padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                h2 { "Prueba de API" }

                button {
                    style: "padding: 10px 20px; background-color: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    disabled: loading(),
                    onclick: fetch_trip,
                    // CORRECCIÓN: Usar expresión directa en lugar de interpolación de cadenas
                    {if loading() { "Cargando..." } else { "Obtener viaje #1" }}
                }

                // Mostrar resultado si existe
                {trip_data().map(|data| {
                    rsx! {
                        div {
                            style: "margin-top: 15px; padding: 10px; border-radius: 4px; background-color: #f0f0f0;",
                            "{data}"
                        }
                    }
                })}
            }
        }
    }
}
