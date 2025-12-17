import { invoke } from "@tauri-apps/api/core";

const tiempo_de_actualizacion = 1500;

// Elementos de texto
let cpu: HTMLElement | null;
let memory: HTMLElement | null;
let disk: HTMLElement | null;
let swap: HTMLElement | null;
let os_name: HTMLElement | null;
let os_version: HTMLElement | null;
let kernel_version: HTMLElement | null;
let host_name: HTMLElement | null;

// Barras de progreso
let rango_cpu: HTMLProgressElement | null;
let rango_memory: HTMLProgressElement | null;
let rango_disk: HTMLProgressElement | null;

window.addEventListener("DOMContentLoaded", () => {
  // Textos
  cpu = document.getElementById("cpu");
  memory = document.getElementById("memory");
  disk = document.getElementById("disk");
  swap = document.getElementById("swap");
  os_name = document.getElementById("os_name");
  os_version = document.getElementById("os_version");
  kernel_version = document.getElementById("kernel_version");
  host_name = document.getElementById("host_name");

  // Barras
  rango_cpu = document.getElementById("rango_cpu") as HTMLProgressElement;
  rango_memory = document.getElementById("rango_memory") as HTMLProgressElement;
  rango_disk = document.getElementById("rango_disk") as HTMLProgressElement;

  // Primera carga
  obtener_informacion_del_sistema();

  // Actualización periódica
  setInterval(obtener_informacion_del_sistema, tiempo_de_actualizacion);
});


// Extrae el valor actual y el total de un string tipo "12.3 / 32.0 GB" o "23.5 / 100 CPUs"
function extraerValores(texto: string): { actual: number, total: number } {

  // 
  const match = texto.match(/([\d.]+)\s*\/\s*([\d.]+)/);// Captura dos números separados por "/"
  if (match) {
    return {
      actual: parseFloat(match[1]), // Primer número
      total: parseFloat(match[2]) // Segundo número
    };
  }
  return { actual: 0, total: 1 }; // Valores por defecto en caso de fallo
}

async function obtener_informacion_del_sistema() {
  try {
    const sysInfo = await invoke<{
      cpu: string;
      memory: string;
      disks: string;
      swap: string;
      os_name: string;
      os_version: string;
      kernel_version: string;
      host_name: string;
      memory_total: number;
      cpu_total: number;
      disks_total: number;
    }>("get_system_info");


    // CPU
    if (cpu && rango_cpu) {
      cpu.textContent = sysInfo.cpu; // Núcleos de mi CPU
      const { actual, total } = extraerValores(sysInfo.cpu); // Extraer valores numéricos
      const porcentaje = total > 0 ? (actual / total) * 100 : 0; // Calcular porcentaje
      rango_cpu.value = porcentaje; // Actualizar barra de progreso
      rango_cpu.max = 100; // Máximo 100%
    }

    // RAM
    if (memory && rango_memory) {
      memory.textContent = sysInfo.memory;
      const { actual, total } = extraerValores(sysInfo.memory);
      const porcentaje = total > 0 ? (actual / total) * 100 : 0;
      rango_memory.value = porcentaje;
      rango_memory.max = 100;
    }

    // Disco
    if (disk && rango_disk) {
      disk.textContent = sysInfo.disks;
      const { actual, total } = extraerValores(sysInfo.disks);
      const porcentaje = total > 0 ? (actual / total) * 100 : 0;
      rango_disk.value = porcentaje;
      rango_disk.max = 100;
    }

    // Otros datos
    if (swap) swap.textContent = sysInfo.swap;
    if (os_name) os_name.textContent = sysInfo.os_name;
    if (os_version) os_version.textContent = sysInfo.os_version;
    if (kernel_version) kernel_version.textContent = sysInfo.kernel_version;
    if (host_name) host_name.textContent = sysInfo.host_name;

  } catch (error) {
    console.error("Error fetching system info:", error);
  }
}
