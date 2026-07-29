//! Reference shell chrome: rail + domain menu + top bar + content host.

use crate::shell::{
    pages_for_domain, tokens, Domain, DomainTier, Page, SettingsTab, ShellNav,
};
use crate::wireframe::draw_wireframe_stub;
use rusterp_api_client::{
    default_rpc_url, live_grpc_supported, live_grpc_unavailable_reason, normalize_rpc_url,
    shared_result, spawn_local_fut, Connection, ConnectionStatus, PartyRow, RefreshSnapshot,
    SharedResult, DEFAULT_RPC_URL, ENDPOINT_ENV, RPC_URL_ENV,
};

/// App version string for Settings → About (workspace package version).
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Minimal shell: chrome + Phase 1 Parties list + Settings panes. No edit/create flows.
pub struct ReferenceApp {
    nav: ShellNav,
    conn: Connection,
    rpc_url: String,
    status: ConnectionStatus,
    parties: Vec<PartyRow>,
    health: Option<String>,
    refresh_slot: Option<SharedResult<RefreshSnapshot>>,
    auto_started: bool,
}

impl ReferenceApp {
    /// Construct the shell. `endpoint_override` comes from CLI when present.
    pub fn new(cc: &eframe::CreationContext<'_>, endpoint_override: Option<String>) -> Self {
        // Embed DejaVu Sans as proportional fallback for Unicode rail icons.
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "DejaVuSans".into(),
            egui::FontData::from_static(include_bytes!("../../../assets/fonts/DejaVuSans.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .push("DejaVuSans".into());
        cc.egui_ctx.set_fonts(fonts);

        let rpc_url = endpoint_override
            .map(|v| normalize_rpc_url(&v))
            .unwrap_or_else(default_rpc_url);

        Self {
            nav: ShellNav::default(),
            conn: Connection::new(rpc_url.clone()),
            rpc_url,
            status: ConnectionStatus::NotConnected,
            parties: Vec::new(),
            health: None,
            refresh_slot: None,
            auto_started: false,
        }
    }

    fn refresh_in_flight(&self) -> bool {
        self.refresh_slot.is_some()
    }

    fn request_refresh(&mut self) {
        if self.refresh_in_flight() {
            return;
        }
        if !live_grpc_supported() {
            self.status = ConnectionStatus::NotConnected;
            self.parties.clear();
            self.health = Some(live_grpc_unavailable_reason().to_string());
            return;
        }

        self.conn.set_url(self.rpc_url.clone());
        self.status = ConnectionStatus::Connecting;
        self.parties.clear();
        self.health = None;

        let slot = shared_result();
        self.refresh_slot = Some(slot.clone());

        let mut conn = Connection::new(self.rpc_url.clone());
        spawn_local_fut(async move {
            let snap = rusterp_api_client::refresh(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(snap));
            }
        });
    }

    fn poll_refresh_slot(&mut self) {
        let Some(slot) = self.refresh_slot.as_ref() else {
            return;
        };
        let maybe = slot.lock().ok().and_then(|g| g.clone());
        let Some(result) = maybe else {
            return;
        };
        self.refresh_slot = None;
        match result {
            Ok(snap) => {
                self.status = snap.status;
                self.parties = snap.parties;
                self.health = snap.health;
            }
            Err(msg) => {
                self.status = ConnectionStatus::error(msg);
                self.parties.clear();
                self.health = None;
            }
        }
    }

    fn apply_dark_style(ctx: &egui::Context) {
        ctx.style_mut_of(egui::Theme::Dark, |style| {
            style.visuals = egui::Visuals::dark();
            style.visuals.window_corner_radius = egui::CornerRadius::ZERO;
            style.visuals.menu_corner_radius = egui::CornerRadius::ZERO;
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::ZERO;
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
            style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
            style.visuals.widgets.active.corner_radius = egui::CornerRadius::ZERO;
            style.visuals.widgets.open.corner_radius = egui::CornerRadius::ZERO;
            style.visuals.selection.bg_fill = tokens::ACCENT;
            style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        });
        ctx.set_theme(egui::Theme::Dark);
    }

    fn draw_rail_domain_button(&mut self, ui: &mut egui::Ui, domain: Domain, selected: Domain) {
        let is_sel = domain == selected;
        let mut text = egui::RichText::new(domain.icon()).size(20.0);
        if is_sel {
            text = text.color(tokens::ACCENT);
        } else if domain.tier() == DomainTier::FutureStub {
            text = text.color(tokens::WIREFRAME_MUTED);
        }
        let response = ui
            .add(egui::Button::new(text).frame(false))
            .on_hover_text(domain.rail_tooltip());
        if is_sel {
            let rect = response.rect;
            let indicator = egui::Rect::from_min_size(
                egui::pos2(rect.left() - 4.0, rect.top()),
                egui::vec2(3.0, rect.height()),
            );
            ui.painter()
                .rect_filled(indicator, egui::CornerRadius::ZERO, tokens::ACCENT);
        }
        if response.clicked() {
            let _ = self.nav.select_domain(domain);
        }
    }

    fn draw_rail(&mut self, ui: &mut egui::Ui) {
        let selected = self.nav.selected_domain;
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            let is_sel = selected == Domain::Settings;
            let mut text = egui::RichText::new(Domain::Settings.icon()).size(20.0);
            if is_sel {
                text = text.color(tokens::ACCENT);
            }
            let response = ui
                .add(egui::Button::new(text).frame(false))
                .on_hover_text(Domain::Settings.rail_tooltip());
            if is_sel {
                let rect = response.rect;
                let indicator = egui::Rect::from_min_size(
                    egui::pos2(rect.left() - 4.0, rect.top()),
                    egui::vec2(3.0, rect.height()),
                );
                ui.painter()
                    .rect_filled(indicator, egui::CornerRadius::ZERO, tokens::ACCENT);
            }
            if response.clicked() {
                self.nav.open_settings();
            }

            ui.add_space(8.0);
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        for domain in Domain::mvp_rail() {
                            self.draw_rail_domain_button(ui, *domain, selected);
                            ui.add_space(4.0);
                        }

                        ui.add_space(4.0);
                        let sep_width = tokens::RAIL_WIDTH - 16.0;
                        let (sep_rect, _) = ui.allocate_exact_size(
                            egui::vec2(sep_width, 1.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().hline(
                            sep_rect.x_range(),
                            sep_rect.center().y,
                            egui::Stroke::new(1.0, tokens::WIREFRAME_MUTED),
                        );
                        ui.add_space(4.0);

                        for domain in Domain::future_rail() {
                            self.draw_rail_domain_button(ui, *domain, selected);
                            ui.add_space(4.0);
                        }
                        ui.add_space(8.0);
                    });
                });
        });
    }

    fn draw_menu(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(self.nav.selected_domain.label().to_uppercase())
                .small()
                .weak(),
        );
        ui.add_space(4.0);

        let pages = pages_for_domain(self.nav.selected_domain);
        for page in pages {
            let page = *page;
            let selected = self.nav.selected_page == page;
            let label = page.label();
            let text = if selected {
                egui::RichText::new(label).strong().color(tokens::ACCENT)
            } else {
                egui::RichText::new(label)
            };
            let row = ui.add_sized(
                [ui.available_width(), tokens::DENSE_ROW],
                egui::Button::new(text).frame(selected),
            );
            if row.clicked() {
                let _ = self.nav.select_page(page);
            }
        }
    }

    fn draw_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            // Logo placeholder (white-label slot).
            ui.add_sized(
                [36.0, 28.0],
                egui::Label::new(egui::RichText::new("RE").strong().color(tokens::ACCENT)),
            );
            ui.separator();
            ui.strong("RustERP");
            ui.label(egui::RichText::new("reference").weak().small());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Flex status / system message region.
                if let Some(msg) = self.status.error_message() {
                    ui.colored_label(tokens::ERROR, msg);
                } else if let Some(h) = &self.health {
                    if self.status == ConnectionStatus::Connected {
                        ui.label(egui::RichText::new(format!("health: {h}")).small().weak());
                    } else if !live_grpc_supported() {
                        ui.label(egui::RichText::new(h.as_str()).small());
                    }
                }

                let status_text = format!("● {}", self.status.as_str());
                let status_color = match &self.status {
                    ConnectionStatus::Connected => egui::Color32::from_rgb(0x81, 0xcf, 0x8d),
                    ConnectionStatus::Connecting => egui::Color32::from_rgb(0xe0, 0xaf, 0x68),
                    ConnectionStatus::Error { .. } => tokens::ERROR,
                    ConnectionStatus::NotConnected => egui::Color32::from_rgb(0xa0, 0xa0, 0xa0),
                };
                ui.label(egui::RichText::new(status_text).color(status_color).strong());
            });
        });
    }

    fn draw_parties_content(&mut self, ui: &mut egui::Ui) {
        ui.heading(self.nav.selected_page.label());
        if let Some(note) = self.nav.customers_suppliers_unfiltered_note() {
            ui.label(egui::RichText::new(note).italics().small());
        }
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            let refresh_enabled = live_grpc_supported() && !self.refresh_in_flight();
            if ui
                .add_enabled(
                    refresh_enabled,
                    egui::Button::new("Refresh").fill(tokens::ACCENT),
                )
                .on_disabled_hover_text(if live_grpc_supported() {
                    "Refresh in progress"
                } else {
                    live_grpc_unavailable_reason()
                })
                .clicked()
            {
                self.request_refresh();
            }
            if matches!(
                self.status,
                ConnectionStatus::Error { .. } | ConnectionStatus::NotConnected
            ) && live_grpc_supported()
            {
                if ui.button("Retry").clicked() {
                    self.request_refresh();
                }
            }
        });
        ui.add_space(8.0);

        match &self.status {
            ConnectionStatus::NotConnected if !live_grpc_supported() => {
                ui.label(live_grpc_unavailable_reason());
            }
            ConnectionStatus::NotConnected => {
                ui.label("Not connected. Click Refresh to call the core.");
            }
            ConnectionStatus::Connecting => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Connecting…");
                });
            }
            ConnectionStatus::Error { message } => {
                ui.colored_label(
                    tokens::ERROR,
                    format!("Connection error: {message}"),
                );
                ui.label("No parties shown (list cleared).");
            }
            ConnectionStatus::Connected if self.parties.is_empty() => {
                ui.label("Connected — no parties on the server (honest empty list).");
            }
            ConnectionStatus::Connected => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("parties_grid")
                        .striped(true)
                        .num_columns(3)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            ui.strong("Display name");
                            ui.strong("Roles");
                            ui.strong("Id");
                            ui.end_row();
                            for p in &self.parties {
                                ui.label(&p.display_name);
                                ui.label(p.roles.join(", "));
                                ui.monospace(&p.id);
                                ui.end_row();
                            }
                        });
                });
            }
        }
    }

    fn draw_settings_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.add_space(4.0);

        // Multi-pane tab strip (only on Settings — never on single-pane pages).
        ui.horizontal(|ui| {
            for tab in SettingsTab::all() {
                let tab = *tab;
                let selected = self.nav.selected_tab == tab;
                if ui
                    .selectable_label(selected, tab.label())
                    .clicked()
                {
                    self.nav.select_tab(tab);
                }
            }
        });
        ui.separator();
        ui.add_space(8.0);

        match self.nav.selected_tab {
            SettingsTab::Connection => self.draw_settings_connection(ui),
            SettingsTab::Modules => self.draw_settings_modules(ui),
            SettingsTab::UsersAndRoles => self.draw_settings_users_roles(ui),
            SettingsTab::About => self.draw_settings_about(ui),
        }
    }

    fn draw_settings_modules(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Modules").strong());
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Static mirror of core.modules seed — synced from platform API when it lands.",
            )
            .small()
            .weak(),
        );
        ui.add_space(8.0);

        struct ModuleRow {
            id: &'static str,
            name: &'static str,
            enabled: bool,
            always_on: bool,
        }

        let rows = [
            ModuleRow {
                id: "core",
                name: "Core Platform",
                enabled: true,
                always_on: true,
            },
            ModuleRow {
                id: "parties",
                name: "Parties",
                enabled: true,
                always_on: false,
            },
            ModuleRow {
                id: "catalog",
                name: "Catalog",
                enabled: true,
                always_on: false,
            },
            ModuleRow {
                id: "sales",
                name: "Sales",
                enabled: true,
                always_on: false,
            },
            ModuleRow {
                id: "payments",
                name: "Payments",
                enabled: true,
                always_on: false,
            },
            ModuleRow {
                id: "inventory",
                name: "Inventory",
                enabled: false,
                always_on: false,
            },
        ];

        egui::Grid::new("modules_grid")
            .striped(true)
            .num_columns(4)
            .min_col_width(80.0)
            .show(ui, |ui| {
                ui.strong("id");
                ui.strong("name");
                ui.strong("enabled");
                ui.strong("always_on");
                ui.end_row();
                for row in rows {
                    ui.monospace(row.id);
                    ui.label(row.name);
                    ui.label(if row.enabled { "yes" } else { "no" });
                    ui.label(if row.always_on { "yes" } else { "no" });
                    ui.end_row();
                }
            });
    }

    fn draw_settings_users_roles(&mut self, ui: &mut egui::Ui) {
        draw_wireframe_stub(
            ui,
            "Users & Roles",
            &crate::shell::WireframeMeta {
                schema_path: "auth.users, auth.roles, auth.permissions",
                tier_label: "MVP schema",
                description: "Classic RBAC — users, groups, roles, resource:action permissions.",
            },
        );
    }

    fn draw_settings_connection(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Connection").strong());
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.label(self.status.as_str());
            if let Some(msg) = self.status.error_message() {
                ui.colored_label(tokens::ERROR, msg);
            }
        });

        if let Some(h) = &self.health {
            if self.status == ConnectionStatus::Connected {
                ui.label(format!("Health: {h}"));
            }
        }

        ui.add_space(8.0);
        ui.label("RPC URL:");
        ui.add(
            egui::TextEdit::singleline(&mut self.rpc_url)
                .desired_width(360.0)
                .hint_text(DEFAULT_RPC_URL),
        );
        ui.label(
            egui::RichText::new(format!(
                "Env: {RPC_URL_ENV} (primary), {ENDPOINT_ENV} (legacy). CLI: --endpoint"
            ))
            .small()
            .weak(),
        );

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let refresh_enabled = live_grpc_supported() && !self.refresh_in_flight();
            if ui
                .add_enabled(
                    refresh_enabled,
                    egui::Button::new("Refresh / Retry").fill(tokens::ACCENT),
                )
                .clicked()
            {
                self.request_refresh();
            }
            if !live_grpc_supported() {
                ui.label(live_grpc_unavailable_reason());
            }
        });
    }

    fn draw_settings_about(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("About").strong());
        ui.add_space(4.0);
        ui.label(format!("RustERP reference UI  v{APP_VERSION}"));
        ui.label("Consumer-only shell — no ERP domain logic or storage in this repo.");
        ui.label("Authority stays on the RustERP core.");
        ui.add_space(8.0);
        ui.hyperlink_to(
            "RustERP core (GitHub)",
            "https://github.com/ndx-video/RustERP",
        );
        ui.hyperlink_to("RustERP.biz", "https://RustERP.biz");
    }

    fn draw_content(&mut self, ui: &mut egui::Ui) {
        ui.set_width(ui.available_width());
        match self.nav.selected_page {
            Page::AllParties | Page::Customers | Page::Suppliers => {
                self.draw_parties_content(ui);
            }
            Page::SettingsHost => {
                self.draw_settings_content(ui);
            }
            page => {
                if let Some(meta) = page.wireframe_meta() {
                    draw_wireframe_stub(ui, page.label(), &meta);
                }
            }
        }
    }
}

impl eframe::App for ReferenceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Self::apply_dark_style(ui.ctx());

        if live_grpc_supported() && !self.auto_started {
            self.auto_started = true;
            self.request_refresh();
        }

        self.poll_refresh_slot();

        if self.refresh_in_flight() {
            ui.ctx().request_repaint();
        }

        // Left icon rail.
        egui::Panel::left("icon_rail")
            .exact_size(tokens::RAIL_WIDTH)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(tokens::SURFACE_RAIL)
                    .inner_margin(egui::Margin::symmetric(4, 0))
                    .corner_radius(egui::CornerRadius::ZERO),
            )
            .show(ui, |ui| {
                self.draw_rail(ui);
            });

        // Domain menu column.
        egui::Panel::left("domain_menu")
            .exact_size(tokens::MENU_WIDTH)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(tokens::SURFACE_MENU)
                    .inner_margin(egui::Margin::symmetric(12, 8))
                    .corner_radius(egui::CornerRadius::ZERO),
            )
            .show(ui, |ui| {
                self.draw_menu(ui);
            });

        // Top bar.
        egui::Panel::top("top_bar")
            .exact_size(tokens::TOP_BAR_HEIGHT)
            .frame(
                egui::Frame::new()
                    .fill(tokens::SURFACE_TOP)
                    .inner_margin(egui::Margin::symmetric(16, 8))
                    .corner_radius(egui::CornerRadius::ZERO),
            )
            .show(ui, |ui| {
                self.draw_top_bar(ui);
            });

        // Content host.
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(tokens::SURFACE)
                    .inner_margin(egui::Margin::same(tokens::PANE_PADDING as i8))
                    .corner_radius(egui::CornerRadius::ZERO),
            )
            .show(ui, |ui| {
                self.draw_content(ui);
            });
    }
}
