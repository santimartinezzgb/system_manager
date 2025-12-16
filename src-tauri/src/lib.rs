use serde::Serialize; // Para serializar estructuras
use sysinfo::{Disks, System};

#[derive(Serialize)] // Para serializar la estructura a JSON
struct SystemInfo {
    // Objeto que contiene la información del sistema
    cpu: String,
    memory: String,
    disks: String,
    swap: String,
    os_name: String,
    os_version: String,
    kernel_version: String,
    host_name: String,
}

#[tauri::command] // Invocable desde el frontend
fn get_system_info() -> SystemInfo {
    // Función que obtiene la información del sistema
    let mut system = System::new_all(); // Crea un nuevo objeto System
    system.refresh_all(); // Refresca toda la información del sistema

    // Crea el objeto CPU
    let cpu = format!(
        "{:.2} / {} CPUs",
        system.global_cpu_usage(),
        system.cpus().len()
    );

    // Crea el objeto memoria
    let memory = format!(
        "{:.2} / {:.2} GB",
        system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    );

    // Crea la información de los discos
    let disks = Disks::new_with_refreshed_list();

    // Obtiene el primer disco y su espacio total
    let storage = if let Some(disks) = disks.get(0) {
        // Si hay al menos un disco
        let total = disks.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        format!("{:.2} GB", total)
    } else {
        "N/A".to_string() // Si no hay discos, devuelve "N/A"
    };

    // Crea el objeto discos
    let disks = format!("Disks: {:.2} GB", storage);

    // Crea el objeto memoria swap
    let swap = format!(
        "{:.2} / {:.2} GB",
        system.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        system.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0
    );

    // OBTENER INFORMACIÓN DEL SISTEMA OPERATIVO
    let os_name = sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string());

    SystemInfo {
        // Devuelve la estructura con toda la información
        cpu,
        memory,
        disks,
        swap,
        os_name,
        os_version,
        kernel_version,
        host_name,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)] // Punto de entrada para móvil
pub fn run() {
    // Función principal para ejecutar la aplicación Tauri
    tauri::Builder::default() // Crea un nuevo constructor de Tauri
        .plugin(tauri_plugin_opener::init()) // Inicializa el plugin opener
        .invoke_handler(tauri::generate_handler![get_system_info]) // Registra el comando invocable
        .run(tauri::generate_context!()) // Ejecuta la aplicación con el contexto generado
        .expect("error while running tauri application"); // Maneja errores al ejecutar la aplicación
}
