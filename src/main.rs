// RUST INJECTOR
// code written by arad and alawapr

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use winapi::{um::{processthreadsapi::OpenProcess, winnt::PROCESS_ALL_ACCESS}, shared::minwindef::FALSE};
use injector::Process;

mod injector;
mod utils;

fn main() -> Result<(), eframe::Error> {

    
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    struct Information {
        process_name: String,
        dll_path: String
    }

    impl Default for Information {
        fn default() -> Self {
            Self {
                process_name: "Process Name".to_owned(),
                dll_path: "Path To DLL".to_owned()
            }
        } 
    }

    let mut information = Information::default();

    eframe::run_simple_native("Injector", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My silly lil injector");
            if ui.button("inject dll").clicked() {
               unsafe {
                injector::inject(
                Process::new(OpenProcess(PROCESS_ALL_ACCESS, FALSE, utils::get_process_id(&information.process_name).unwrap())), 
                injector::Path::new(widestring::U16CString::from_str(&information.dll_path).unwrap())
                );
               }
               
            }

            ui.text_edit_multiline(&mut information.process_name);
            ui.text_edit_multiline(&mut information.dll_path);
           
        });
    })
}