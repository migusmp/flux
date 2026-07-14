```
███████╗██╗     ██╗   ██╗██╗  ██╗
██╔════╝██║     ██║   ██║╚██╗██╔╝
█████╗  ██║     ██║   ██║ ╚███╔╝
██╔══╝  ██║     ██║   ██║ ██╔██╗
██║     ███████╗╚██████╔╝██╔╝ ██╗
╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝
```

<div align="center">

### DevOps monitoring and logging TUI

*Your entire VPS state, in a single terminal.*

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/status-en%20desarrollo-orange?style=for-the-badge)

</div>

---

## ¿Qué es Flux?

What is Flux?

Flux is a TUI (Terminal User Interface) tool written in Rust, designed to monitor processes, containers, and logs from a VPS server without leaving the terminal. It was born from a very specific need: having the real-time state of your services (CPU, RAM, Docker containers, logs) available with the same speed as opening htop, but tailored for modern DevOps stacks.

No heavy browser dashboards. A single binary, a terminal, and your infrastructure information right in front of you.

## Features

- 📊 Real-time monitoring — CPU, RAM, disk, and swap usage of the host
- 🐳 Docker containers — status, resource usage, and container restarts
- 📜 Live logs — tail, filtering, and searching without leaving the TUI
- ⚙️ Declarative configuration — define what to monitor in a .yaml or .config file
- 🔄 Background mode (daemon) — Flux keeps collecting data even after closing the TUI, and can send it to your own backend or frontend
- 🔔 Configurable alerts — threshold-based notifications (e.g. RAM above 80%) directly in the interface

## Installation

```bash
cargo install flux-tui
```

*(Coming soon to crates.io)*

## Quick start

```bash
# Start the TUI using your configuration
flux --config flux.yaml

# Start only the background daemon
flux daemon start --config flux.yaml
```

## Configuration

Flux is configured through a `.yaml` file where you define which logs and processes you want to monitor:

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

Flux development is progressing in phases, from a basic local monitor to remote communication between multiple Flux instances (without relying on VPNs or manually configured tunnels). Check the complete roadmap in [`flux_roadmap.md`](./flux_roadmap.md).

## Stack técnico

| Componente | Tecnología |
|---|---|
| TUI interface | `ratatui` + `crossterm` |
| System metrics | `sysinfo` |
| Docker integration | `bollard` |
| Configuration | `serde` + `serde_yaml` |
| Local persistence | `rusqlite` |

## License

MIT

---

<div align="center">
<sub>Built with 🦀 Rust, for those who live in the terminal.</sub>
</div>