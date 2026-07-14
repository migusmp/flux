# Flux — Roadmap de desarrollo

TUI de monitorización y logs para DevOps, escrita en Rust.

---

## Fase 0 — Fundamentos (setup del proyecto)

- Estructura del proyecto en Rust (workspace con crates separados: `core`, `tui`, `daemon`, `cli`)
- Elegir stack de TUI: `ratatui` (sucesor de tui-rs, muy activo) + `crossterm` como backend
- Definir el modelo de datos base: `Metric`, `LogEntry`, `Target` (servidor/contenedor a monitorizar)
- Parser de configuración `.yaml` (usando `serde_yaml`)
- Logging propio de la app (para debug de Flux, no confundir con los logs que monitoriza)

**Objetivo de la fase:** tener el esqueleto compilando con una TUI vacía que lee un `.yaml` de ejemplo.

---

## Fase 1 — MVP local (monitor de un solo host)

- Vista de sistema: CPU, RAM, disco, swap en tiempo real (crate `sysinfo`)
- Vista de contenedores Docker: estado, uso CPU/RAM por contenedor (via socket de Docker, crate `bollard`)
- Panel de logs con tail en vivo de un archivo o `docker logs -f`
- Filtro básico por texto/regex dentro del panel de logs
- Navegación entre paneles con teclado (estilo `btop`/`k9s`)
- Config `.yaml` mínima: qué contenedores y qué rutas de log observar

**Objetivo de la fase:** Flux corriendo standalone en tu VPS de Hetzner, sustituyendo a `docker stats` + `tail -f` manuales.

```yaml
# ejemplo config v1
targets:
  - name: materialtrack-app
    type: docker
    container: materialtrack_app
  - name: caddy
    type: file
    path: /var/log/caddy/access.log
```

---

## Fase 2 — Acciones y usabilidad

- Restart/stop/start de contenedores desde la TUI (con confirmación)
- Ejecutar scripts propios desde atajos de teclado (`backup.sh`, `update.sh`, tu cron de reinicio)
- Highlighting de logs por nivel (error/warn/info) con colores
- Búsqueda dentro del histórico de logs (no solo tail en vivo)
- Alertas por umbral configurable en el `.yaml` (ej. RAM > 80%)
- Notificación visual dentro de la TUI cuando salta una alerta

**Objetivo de la fase:** Flux deja de ser "solo lectura" y se convierte en herramienta de acción rápida.

---

## Fase 3 — Modo daemon + persistencia

- Modo background (`flux daemon start`) que corre independiente de la TUI
- Histórico de métricas en SQLite local (para consultar picos pasados sin depender de nada externo)
- Exportación opcional a formato compatible con Prometheus (endpoint `/metrics`)
- Notificaciones externas por webhook (Telegram/Discord) cuando salta una alerta, incluso sin la TUI abierta
- La TUI pasa a ser un **cliente** que se conecta al daemon local vía socket Unix o puerto local

**Objetivo de la fase:** separar "recolección" (daemon) de "visualización" (TUI), como Prometheus + Grafana pero en una sola herramienta.

---

## Fase 4 — Conexión remota entre instancias de Flux

Esta es la parte que te interesa especialmente. La idea: no depender de Tailscale ni de SSH manual, sino que **Flux hable con Flux**.

- El daemon expone un servidor (gRPC o un protocolo propio ligero sobre TCP/TLS) además del socket local
- Autenticación por par de claves (estilo SSH: generas un keypair al instalar Flux, añades la pública del cliente al servidor)
- Desde la TUI: `flux connect usuario@host:puerto` o un selector de "hosts conocidos" guardado en config
- Multiplexado: una misma TUI puede tener pestañas/paneles conectados a varios daemons remotos a la vez (tu VPS de Hetzner + tu Raspberry Pi + lo que añadas después)
- Descubrimiento opcional en red local (mDNS) para detectar instancias de Flux en la misma LAN sin configurar IP a mano
- Fallback: si el daemon remoto no está corriendo, poder lanzar Flux remotamente vía SSH como respaldo (reutilizando credenciales SSH ya configuradas, no como método principal)

**Objetivo de la fase:** desde tu Arch con Hyprland, abrir Flux y ver de un vistazo el estado de Hetzner + Raspberry Pi + cualquier otro host con Flux instalado, sin túneles manuales.

```yaml
# ejemplo config v4 — hosts remotos
remotes:
  - name: hetzner-vps
    host: cra-pzh.es
    port: 7878
    key: ~/.config/flux/keys/hetzner.pub
  - name: raspberry-home
    host: raspberrypi.local
    port: 7878
    discovery: mdns
```

---

## Fase 5 — Pulido y extras

- Export de sesión de logs a archivo (para debugging offline o adjuntar en un issue)
- Modo "solo diffs" en logs muy verbosos (no repetir líneas idénticas)
- Temas de color configurables (transparencias, estilo tu setup actual de Hyprland/Kitty)
- Plugins/scripts custom por usuario (comandos rápidos definidos por el propio usuario en el `.yaml`)
- Documentación + `cargo install flux-tui` publicado en crates.io

---

## Resumen de dependencias clave (Rust)

| Función | Crate sugerido |
|---|---|
| TUI | `ratatui` + `crossterm` |
| Sistema (CPU/RAM/disco) | `sysinfo` |
| Docker | `bollard` |
| Config YAML | `serde` + `serde_yaml` |
| Base de datos local | `rusqlite` |
| Servidor remoto (daemon) | `tonic` (gRPC) o `tokio` + `quinn` (QUIC) |
| Descubrimiento LAN | `mdns-sd` |
| Async runtime | `tokio` |

---

## Orden recomendado de ataque

1. Fase 0 y 1 primero, en tu propio VPS — así tienes algo usable rápido y feedback real
2. Fase 2 en cuanto la vista básica funcione bien, porque las acciones rápidas son las que más valor dan en el día a día
3. Fase 3 antes que la 4 — el daemon local es requisito técnico para que la conexión remota (Fase 4) tenga sentido
4. Fase 4 es la más compleja (protocolo propio + auth), resérvala para cuando ya domines bien el resto del stack async de Rust (`tokio`)