# Frontend Local Rust OS

Este proyecto es una aplicación de escritorio desarrollada con Rust y Dioxus que funciona como frontend para el backend desplegado en [https://github.com/juanloaiza21/rust-backend-os](https://github.com/juanloaiza21/rust-backend-os). La aplicación permite visualizar y consultar datos de viajes de taxis a través de diferentes filtros y criterios.

## Características principales

- Interfaz de usuario desarrollada con Dioxus
- Consultas a API REST
- Visualización de datos en formato tabular
- Múltiples criterios de búsqueda (por índice, rango de precios, destino)
- Diseño responsive y amigable

## Capturas de pantalla

![Captura de pantalla de la aplicación](assets/screenshot.png)

## Requisitos previos

Para compilar y ejecutar esta aplicación, necesitarás:

- [Rust](https://www.rust-lang.org/tools/install) (versión 1.60 o superior)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (incluido con Rust)
- [Dioxus CLI](https://dioxuslabs.com/docs/0.6/guide/en/getting_started/cli.html) (opcional pero recomendado)

### Dependencias específicas por sistema operativo

#### Linux
```bash
# Para distribuciones basadas en Debian/Ubuntu
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev

# Para Fedora
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

#### macOS
```bash
brew install gtk+3 webkit2gtk
```

#### Windows
No se requieren dependencias adicionales específicas.

## Instalación

1. Clona el repositorio:
```bash
git clone https://github.com/juanloaiza21/frontend-local-rust-os.git
cd frontend-local-rust-os
```

2. Instala la CLI de Dioxus (opcional pero recomendado):
```bash
cargo install dioxus-cli
```

## Compilación y ejecución

### Usando Cargo

Para desarrollo:
```bash
# Compilar en modo desarrollo
cargo build

# Ejecutar en modo desarrollo
cargo run
```

Para producción:
```bash
# Crear una versión optimizada para distribución
cargo build --release

# Ejecutar en modo producción
cargo run --release
```

### Usando Dioxus CLI

Para desarrollo:
```bash
# Iniciar servidor de desarrollo
dx serve --platform desktop
```

Para producción:
```bash
# Compilar para producción
dx build --release --platform desktop
```

## Estructura del proyecto

```
frontend-local-rust-os/
├─ assets/           # Recursos estáticos (imágenes, etc.)
├─ src/              # Código fuente
│  ├─ api/           # Módulos para comunicación con API
│  │  ├─ apicalls.rs # Implementación de llamadas a la API
│  │  ├─ mod.rs      # Módulo API
│  ├─ main.rs        # Punto de entrada y componentes principales
├─ Cargo.toml        # Configuración y dependencias del proyecto
├─ README.md         # Documentación
```

## Componentes principales

### App Component

El componente principal de la aplicación que maneja:
- Estados de los formularios
- Comunicación con la API
- Renderizado de datos
- Paginación de resultados

### API Calls

El módulo `api::apicalls` implementa la comunicación con el backend, incluyendo:

- `get_by_index` - Consulta un viaje por su ID
- `get_by_price_range` - Consulta viajes dentro de un rango de precios
- `get_by_destination` - Consulta viajes filtrados por destino

## Variables de entorno

La aplicación utiliza las siguientes variables de entorno para resolver problemas gráficos:

- `GDK_BACKEND=x11`
- `LIBGL_ALWAYS_SOFTWARE=1`
- `WEBKIT_DISABLE_COMPOSITING_MODE=1`

## Solución de problemas comunes

### Error al compilar en Linux

Si encuentras errores relacionados con GTK o WebKit:
```bash
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev
```

### Problemas de renderizado

Si la interfaz no se muestra correctamente, asegúrate de que las variables de entorno están configuradas como se menciona arriba.

### Error de conexión con la API

La aplicación intenta conectarse a `https://backend-rust-277582128315.us-central1.run.app/`. Asegúrate de que:
1. Tienes conexión a Internet
2. El backend está en funcionamiento
3. No hay restricciones de firewall que impidan la conexión

## Contribución

Las contribuciones son bienvenidas. Por favor:

1. Haz fork del proyecto
2. Crea una rama para tu función (`git checkout -b feature/amazing-feature`)
3. Haz commit de tus cambios (`git commit -m 'Add some amazing feature'`)
4. Push a la rama (`git push origin feature/amazing-feature`)
5. Abre un Pull Request

## Licencia

Distribuido bajo la Licencia MIT. Ver `LICENSE` para más información.

## Contacto

Juan Loaiza - [@juanloaiza21](https://github.com/juanloaiza21)

Enlace del proyecto: [https://github.com/juanloaiza21/frontend-local-rust-os](https://github.com/juanloaiza21/frontend-local-rust-os)
