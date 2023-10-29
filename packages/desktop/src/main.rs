// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{NetworkExt, Networks, ProcessExt, System, SystemExt};
use tauri::{AppHandle, Manager};

mod user_config;
mod utils;

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

#[tauri::command]
fn read_config_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String, String> {
  user_config::read_file(config_path_override, app_handle)
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn test() -> Result<String, String> {
  // Please note that we use "new_all" to ensure that all list of
  // components, network interfaces, disks and users are already
  // filled!
  let mut sys = System::new_all();

  // First we update all information of our `System` struct.
  sys.refresh_all();

  println!("=> system:");
  // RAM and swap information:
  println!("total memory: {} bytes", sys.total_memory());
  println!("used memory : {} bytes", sys.used_memory());
  println!("total swap  : {} bytes", sys.total_swap());
  println!("used swap   : {} bytes", sys.used_swap());

  // Display system information:
  // sys.ho
  println!("System name:             {:?}", sys.name());
  println!("System kernel version:   {:?}", sys.kernel_version());
  println!("System OS version:       {:?}", sys.os_version());
  println!("System host name:        {:?}", sys.host_name());

  // Number of CPUs:
  println!("NB CPUs: {}", sys.cpus().len());

  // Display processes ID, name na disk usage:
  // for (pid, process) in sys.processes() {
  //   println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
  // }

  // We display all disks' information:
  println!("=> disks:");
  // let mut disks = Disks::new();
  // We refresh the disk list.
  for disk in sys.disks() {
    println!("{disk:?}");
  }

  // We refresh the network interface list.
  println!("=> networks:");
  for (interface_name, data) in sys.networks() {
    println!(
      "{interface_name}: {}/{} B",
      data.received(),
      data.transmitted()
    );
  }

  // Components temperature:
  // We refresh the component list.
  println!("=> components:");
  for component in sys.components() {
    println!("{component:?}");
  }

  Ok("aaaa".into())
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          println!("{:?}", matches);
        }
        Err(_) => panic! {"CLI Parsing Error"},
      };
      Ok(())
    })
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      app
        .emit_all("single-instance", Payload { args: argv, cwd })
        .unwrap();
    }))
    .invoke_handler(tauri::generate_handler![read_config_file, test])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application.");
}
