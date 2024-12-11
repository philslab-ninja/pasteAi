use serde_json::json;
use std::process::exit;
use tauri::AppHandle;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_store::StoreExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn get_system_prompt_from_settings(app: AppHandle) -> String {
    let default_prompt = "You are a grammar and language corrector, you will write better sentences. You will not change the language of the sentence, only make it better. You do not answer any questions.";
    let store = app.store("store.json").expect("Failed to access store");

    let system_prompt = store.get("system_prompt").unwrap_or(json!(""));

    if system_prompt.as_str().unwrap_or("").len() < 10 {
        return default_prompt.to_string();
    } else {
        return system_prompt.as_str().unwrap_or("").to_string();
    }
}

#[tauri::command]
fn set_system_prompt_from_settings(app: AppHandle, prompt: String) {
    let store = app.store("store.json").expect("Failed to access store");
    store.set("system_prompt", prompt);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]), /* arbitrary number of args to pass to your app */
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|_app| {
            #[cfg(target_os = "macos")]
            _app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;
            //todo -> store.close_resource();

            Ok(())
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {
            // Handle second instance here
            exit(0);
        }))
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_system_prompt_from_settings,
            set_system_prompt_from_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
