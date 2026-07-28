//! Native binary entrypoint for the RustERP reference UI shell.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusterp_ui::ReferenceApp;

/// Optional `--endpoint <uri>` (native). Env `RUSTERP_RPC_URL` / `RUSTERP_GRPC_ENDPOINT` also work.
fn endpoint_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--endpoint" {
            return args.next();
        }
        if let Some(v) = a.strip_prefix("--endpoint=") {
            return Some(v.to_string());
        }
    }
    None
}

fn main() -> eframe::Result {
    env_logger::init();

    // Enter a tokio runtime so slozhn/tonic background tasks can spawn (Macaron pattern).
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("tokio runtime");
    let _guard = rt.enter();

    let endpoint = endpoint_from_args();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([800.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RustERP Reference UI",
        native_options,
        Box::new(move |cc| Ok(Box::new(ReferenceApp::new(cc, endpoint)))),
    )
}
