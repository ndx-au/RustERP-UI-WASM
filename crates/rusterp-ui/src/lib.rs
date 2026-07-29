//! Reference UI shell for RustERP — presentation only; no domain logic.

mod app;
mod shell;
mod wireframe;

pub use app::ReferenceApp;
pub use shell::{Domain, Page, SettingsTab, ShellNav};

/// Shared app factory used by native and WASM shells.
pub fn start_app(cc: &eframe::CreationContext<'_>) -> Box<dyn eframe::App> {
    Box::new(ReferenceApp::new(cc, None))
}

// ---- WASM entry (trunk builds the lib as cdylib) ----
#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn wasm_main() {
        console_error_panic_hook::set_once();
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        let web_options = eframe::WebOptions::default();
        wasm_bindgen_futures::spawn_local(async {
            let document = web_sys::window()
                .expect("window")
                .document()
                .expect("document");
            if let Some(el) = document.get_element_by_id("loading") {
                el.set_text_content(Some("Starting egui…"));
            }

            let canvas = document
                .get_element_by_id("the_canvas_id")
                .expect("the_canvas_id")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("canvas element");

            let start_result = eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(|cc| Ok(crate::start_app(cc))),
                )
                .await;

            if let Some(el) = document.get_element_by_id("loading") {
                match &start_result {
                    Ok(_) => {
                        let _ = el.set_attribute("style", "display:none");
                    }
                    Err(e) => {
                        let _ = el.set_attribute("style", "pointer-events: auto;");
                        el.set_inner_html(&graphics_init_failed_html(&format!("{e:?}")));
                    }
                }
            }
        });
    }

    fn graphics_init_failed_html(err_str: &str) -> String {
        format!(
            r#"<div style="max-width: 480px; padding: 24px; background: #1c1f26; border: 1px solid #ff5555; border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.6); text-align: center; color: #e6e6e6;">
  <h3 style="margin-top: 0; color: #ff5555; font-size: 18px;">Graphics Initialization Failed</h3>
  <p style="font-size: 14px; line-height: 1.5; color: #d0d0d0; margin-bottom: 15px;">
    RustERP Reference UI is a full-client WASM application rendering directly to a GPU-accelerated canvas. We could not start the graphics runner.
  </p>

  <div style="text-align: left; font-size: 13px; background: #222530; border: 1px solid #e0af68; padding: 12px; border-radius: 6px; margin: 15px 0; color: #e0af68; line-height: 1.4;">
    <strong style="display: block; margin-bottom: 4px;">⚠️ Canvas Permission Required:</strong>
    Privacy browsers (like LibreWolf, Tor, or Brave) restrict canvas read/write by default.
    <strong>Please check the right side of your address bar</strong> for a small canvas/warning icon, click it, and select <strong>Allow</strong>. See the <a href="https://librewolf.net/docs/faq/" target="_blank" style="color: #80bfff; text-decoration: underline;">LibreWolf FAQ</a>.
  </div>

  <div style="text-align: left; font-size: 13px; background: #111317; padding: 12px; border-radius: 6px; border: 1px solid #2e3440; margin: 15px 0; color: #c0caf5;">
    <strong style="display: block; margin-bottom: 6px; color: #ff9e64;">Other Troubleshooting Options:</strong>
    <ul style="margin: 0; padding-left: 20px; line-height: 1.4;">
      <li><strong>WebGL 2 Configurations:</strong> Open <code>about:config</code>, set <code>webgl.disabled</code> to <code>false</code>, and reload.</li>
      <li><strong>Hardware Acceleration:</strong> Ensure "Use hardware acceleration when available" is turned on in your browser's settings.</li>
      <li><strong>Brave Shields:</strong> Set fingerprint blocking to standard/disabled to allow canvas rendering.</li>
    </ul>
  </div>
  <p style="font-size: 11px; color: #707580; margin-bottom: 0; word-break: break-all;">Error: {err_str}</p>
</div>"#
        )
    }
}
