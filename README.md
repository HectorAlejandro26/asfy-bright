# asfy-bright

Nacido como un *spin-off* directo de mi otro proyecto [`asfy-vol`](https://github.com/HectorAlejandro26/asfy-vol), `asfy-bright` recicla lo mejor de su arquitectura para traerte un indicador de brillo visual. Está desarrollado en Rust utilizando GTK4 y gtk4-layer-shell. Está diseñado para entornos de escritorio que soportan el protocolo Layer Shell (como Wayland/Sway/Hyprland), proporcionando una barra de brillo minimalista que responde a eventos del sistema.

## Características

- Interfaz construida con GTK4
- Soporte para Layer Shell (flota sobre otras ventanas)
- Configurable mediante archivos TOML
- Manejo de umbrales de iconos dinámicos según el nivel de brillo

## Instalación

Para compilar el proyecto desde el código fuente, asegúrate de tener instalado el toolchain de Rust y las dependencias de desarrollo de GTK4.

```bash
git clone https://github.com/HectorAlejandro26/asfy-bright.git
cd asfy-bright
makepkg -si
```
## Dependencias principales

- `gtk4`
- `gtk4-layer-shell`
- `glibc`
- `gcc-libs`
- `brightnessctl`

## Configuración

El programa busca su archivo de configuración en `$XDG_CONFIG_HOME/asfy/asfy-bright/config.toml`. Ejemplo de `config.toml`:

```toml
use_percent = false

[[thresholds]]
icon = "󰃚"
level = 0.1425

[[thresholds]]
icon = "󰃛"
level = 0.1425

[[thresholds]]
icon = "󰃜"
level = 0.1425

[[thresholds]]
icon = "󰃝"
level = 0.1425

[[thresholds]]
icon = "󰃞"
level = 0.1425

[[thresholds]]
icon = "󰃟"
level = 0.1425

[[thresholds]]
icon = "󰃠"
level = 0.1425
```

**Parámetros:**
- `use_percent`: Determina si se muestra el porcentaje de brillo.
- `thresholds`: Lista de iconos y sus respectivos niveles de brillo (0.0 a 1.0) para cambiar el icono según el nivel actual.
- `style_path`: Ruta del archivo CSS.

## Licencia

Este proyecto está bajo la **Licencia MIT**. Consulta el archivo [LICENSE](LICENSE) para más detalles.
