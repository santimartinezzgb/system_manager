import { invoke } from "@tauri-apps/api/core";
// Invoke. Rust function defined in src/main.rs
// Esta función se encarga de obtener la información
// del sistema desde el backend de Rust

let tiempo_de_actualizacion = 1500;

let cpu: HTMLElement | null;
let memory: HTMLElement | null;
let disk: HTMLElement | null;
let swap: HTMLElement | null;
let os_name: HTMLElement | null;
let os_version: HTMLElement | null;
let kernel_version: HTMLElement | null;
let host_name: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {

  cpu = document.getElementById("cpu");
  memory = document.getElementById("memory");
  disk = document.getElementById("disk");
  swap = document.getElementById("swap");
  os_name = document.getElementById("os_name");
  os_version = document.getElementById("os_version");
  kernel_version = document.getElementById("kernel_version");
  host_name = document.getElementById("host_name");

  // Initial fetch
  obtener_informacion_del_sistema();

  // Obtener la información del sistema
  setInterval(obtener_informacion_del_sistema, tiempo_de_actualizacion);

});

async function obtener_informacion_del_sistema() {
  try {
    const sysInfo = await invoke<{
      cpu: string;
      memory: string;
      disk: string;
      swap: string;
      os_name: string;
      os_version: string;
      kernel_version: string;
      host_name: string;
    }>("get_system_info");

    // Poner los datos del sistema en el HTML
    if (cpu) cpu.textContent = sysInfo.cpu;
    if (memory) memory.textContent = sysInfo.memory;
    if (disk) disk.textContent = sysInfo.disk;
    if (swap) swap.textContent = sysInfo.swap;
    if (os_name) os_name.textContent = sysInfo.os_name;
    if (os_version) os_version.textContent = sysInfo.os_version;
    if (kernel_version) kernel_version.textContent = sysInfo.kernel_version;
    if (host_name) host_name.textContent = sysInfo.host_name;

  } catch (error) {
    console.error("Error fetching system info:", error);
  }
}
