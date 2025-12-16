// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Oculta la consola en Windows para release

fn main() {
    // Punto de entrada principal
    system_manager_lib::run() // Llama a la función run del módulo system_manager_lib
}
