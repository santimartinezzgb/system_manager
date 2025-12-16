# System Manager

Aplicación de escritorio multiplataforma construida con Tauri, Vite (TypeScript) y Rust.

Muestra la información estática y dinámica del sistema en directo.

## Estructura del proyecto

- **src/**: Código fuente del frontend (Vite + TS + CSS)
- **src-tauri/**: Backend en Rust y configuración de Tauri
- **index.html**: Entrada principal de la app
- **start.sh**: Script para desarrollo


## Instalación

```bash
bun install
cd src-tauri && cargo build
```


## Desarrollo

```bash
./start.sh
```
Esto inicia el frontend (bun) y el backend en modo desarrollo.

## Producción

```bash
cd src-tauri
cargo tauri build
```
Genera el ejecutable de escritorio.

## Autor

Santi
