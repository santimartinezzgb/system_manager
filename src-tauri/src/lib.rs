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
    memory_total: f64,
    cpu_total: f64,
    disks_total: f64,
}

#[tauri::command] // Invocable desde el frontend
fn get_system_info() -> SystemInfo {
    // Función que obtiene la información del sistema
    let mut system = System::new_all(); // Crea un nuevo objeto System
    system.refresh_all(); // Refresca toda la información del sistema

    // Crea el objeto CPU en formato 'núcleos_utilizados / núcleos totales'
    let cpu_total = system.cpus().len() as f64;
    let cpu_used = (system.global_cpu_usage() / 100.0) * cpu_total as f32;
    let cpu = format!("{:.2} / {:.0} CPUs", cpu_used, cpu_total);

    // Crea el objeto memoria
    let memory = format!(
        "{:.2} / {:.2} GB",
        system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    );

    // Crea la información de los discos
    let disks = Disks::new_with_refreshed_list();

    // Obtiene el primer disco y su espacio total y usado
    let (disk_used, disk_total) = if let Some(disk) = disks.get(0) {
        let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used = (disk.total_space() - disk.available_space()) as f64 / 1024.0 / 1024.0 / 1024.0;
        (used, total)
    } else {
        (0.0, 1.0)
    };

    // Crea el objeto discos en formato "usado / total GB"
    let disks = format!("{:.2} / {:.2} GB", disk_used, disk_total);

    // Crea el objeto memoria swap
    let swap = format!(
        "{:.2} / {:.2} GB",
        system.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        system.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0
    );

    // Valores totales de memoria, cpu y discos
    let memory_total = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let cpu_total = system.cpus().len() as f64;
    let disks_total = disk_total;

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
        memory_total,
        cpu_total,
        disks_total,
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
