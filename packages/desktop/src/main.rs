// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};

mod providers;
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
fn listen_provider() {
  //
  // Have a provider "manager/scheduler"?
  // Or initialize provider directly on listen?
  //   if initializing directly, how to
}

struct AsyncProcInputTx {
  inner: Mutex<mpsc::Sender<String>>,
}

fn main() {
  tracing_subscriber::fmt::init();

  let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
  let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

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
    .setup(|app| {
      let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("bleep-backend")
        .build()
        .unwrap();

      app.manage(runtime);

      tauri::async_runtime::spawn(async move {
        async_process_model(async_proc_input_rx, async_proc_output_tx).await
      });

      let app_handle = app.handle();
      tauri::async_runtime::spawn(async move {
        loop {
          if let Some(output) = async_proc_output_rx.recv().await {
            rs2js(output, &app_handle);
          }
        }
      });

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

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
  info!(?message, "rs2js");
  manager
    .emit_all("rs2js", format!("rs: {}", message))
    .unwrap();
}

#[tauri::command]
async fn js2rs(
  message: String,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
  info!(?message, "js2rs");
  let async_proc_input_tx = state.inner.lock().await;
  async_proc_input_tx
    .send(message)
    .await
    .map_err(|e| e.to_string())
}

async fn async_process_model(
  mut input_rx: mpsc::Receiver<String>,
  output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  while let Some(input) = input_rx.recv().await {
    let output = input;
    output_tx.send(output).await?;
  }

  Ok(())
}
