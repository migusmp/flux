```
███████╗██╗     ██╗   ██╗██╗  ██╗
██╔════╝██║     ██║   ██║╚██╗██╔╝
█████╗  ██║     ██║   ██║ ╚███╔╝
██╔══╝  ██║     ██║   ██║ ██╔██╗
██║     ███████╗╚██████╔╝██╔╝ ██╗
╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝
```

<div align="center">

### TUI de monitorización y logs para DevOps

*Todo el estado de tu VPS, en una sola terminal.*

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/status-en%20desarrollo-orange?style=for-the-badge)

</div>

---

## ¿Qué es Flux?

**Flux** es una herramienta TUI (Terminal User Interface) escrita en Rust, pensada para monitorizar procesos, contenedores y logs de un servidor VPS sin salir de la terminal. Nace de una necesidad muy concreta: tener a mano el estado real de tus servicios (CPU, RAM, contenedores Docker, logs) con la misma rapidez con la que abrirías `htop`, pero orientado a stacks de DevOps modernos.

Nada de dashboards pesados en el navegador. Un solo binario, una terminal, y todo el flujo de información de tu infraestructura delante.

## Características

- 📊 **Monitorización en tiempo real** — CPU, RAM, disco y swap del host
- 🐳 **Contenedores Docker** — estado, consumo de recursos y reinicios por contenedor
- 📜 **Logs en vivo** — tail, filtrado y búsqueda sin salir de la TUI
- ⚙️ **Configuración declarativa** — define qué observar en un archivo `.yaml` o `.config`
- 🔄 **Modo background (daemon)** — Flux sigue recolectando datos aunque cierres la TUI, y puede enviarlos a un backend o frontend propio
- 🔔 **Alertas configurables** — avisos por umbral (ej. RAM al 80%) directamente en la interfaz

## Instalación

```bash
cargo install flux-tui
```

*(Próximamente disponible en crates.io)*

## Uso rápido

```bash
# Arrancar la TUI apuntando a tu configuración
flux --config flux.yaml

# Arrancar solo el daemon en segundo plano
flux daemon start --config flux.yaml
```

## Configuración

Flux se configura mediante un archivo `.yaml` donde defines qué logs y qué procesos quieres observar:

```yaml
targets:
  - name: app-backend
    type: docker
    container: app_backend

  - name: nginx
    type: file
    path: /var/log/nginx/access.log

alerts:
  - target: app-backend
    metric: memory
    threshold: 80%
```

## Roadmap

El desarrollo de Flux avanza por fases, desde el monitor local básico hasta la conexión remota entre distintas instancias de Flux (sin depender de VPN ni túneles manuales). Consulta el roadmap completo en [`flux-roadmap.md`](./flux-roadmap.md).

## Stack técnico

| Componente | Tecnología |
|---|---|
| Interfaz TUI | `ratatui` + `crossterm` |
| Métricas de sistema | `sysinfo` |
| Integración Docker | `bollard` |
| Configuración | `serde` + `serde_yaml` |
| Persistencia local | `rusqlite` |

## Licencia

MIT

---

<div align="center">
<sub>Construido con 🦀 Rust, para quienes viven en la terminal.</sub>
</div>