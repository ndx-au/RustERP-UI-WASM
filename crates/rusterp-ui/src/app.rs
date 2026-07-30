//! Reference shell chrome: rail + domain menu + top bar + content host.

use crate::shell::{
    pages_for_domain, tokens, Domain, DomainTier, Page, SettingsTab, ShellNav,
};
use crate::wireframe::draw_wireframe_stub;
use rusterp_api_client::{
    add_address, add_contact, create_party, default_rpc_url, list_addresses, list_contacts,
    live_grpc_supported, live_grpc_unavailable_reason, normalize_rpc_url, shared_result,
    spawn_local_fut, AddressRow, AllocationRow, BankAccountRow, CategoryRow, Connection,
    ConnectionStatus, ContactRow, ModuleRow, PartyRole, PartyRow, PaymentRow, PermissionRow,
    ProductRow, RefreshSnapshot, RoleRow, SalesDocRow, SharedResult, StockLevelRow, StockMoveRow,
    UserRow, WarehouseRow, DEFAULT_RPC_URL, ENDPOINT_ENV, RPC_URL_ENV,
};

/// App version string for Settings → About (workspace package version).
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Shell: chrome + live MVP domains + Settings panes.
pub struct ReferenceApp {
    pub(crate) nav: ShellNav,
    pub(crate) conn: Connection,
    pub(crate) rpc_url: String,
    pub(crate) status: ConnectionStatus,
    pub(crate) parties: Vec<PartyRow>,
    pub(crate) health: Option<String>,
    pub(crate) refresh_slot: Option<SharedResult<RefreshSnapshot>>,
    pub(crate) auto_started: bool,
    // Create-party form
    pub(crate) new_party_name: String,
    pub(crate) new_party_customer: bool,
    pub(crate) new_party_supplier: bool,
    pub(crate) new_party_prospect: bool,
    pub(crate) form_error: Option<String>,
    pub(crate) mutate_slot: Option<SharedResult<Result<(), String>>>,
    // Contacts / addresses
    pub(crate) selected_party_id: Option<String>,
    pub(crate) contacts: Vec<ContactRow>,
    pub(crate) addresses: Vec<AddressRow>,
    pub(crate) new_contact_name: String,
    pub(crate) new_contact_email: String,
    pub(crate) new_contact_phone: String,
    pub(crate) new_address_line1: String,
    pub(crate) new_address_city: String,
    pub(crate) new_address_country: String,
    pub(crate) contacts_slot: Option<SharedResult<Result<Vec<ContactRow>, String>>>,
    pub(crate) addresses_slot: Option<SharedResult<Result<Vec<AddressRow>, String>>>,
    // Catalog
    pub(crate) products: Vec<ProductRow>,
    pub(crate) categories: Vec<CategoryRow>,
    pub(crate) new_product_sku: String,
    pub(crate) new_product_name: String,
    pub(crate) new_category_name: String,
    pub(crate) products_slot: Option<SharedResult<Result<Vec<ProductRow>, String>>>,
    pub(crate) categories_slot: Option<SharedResult<Result<Vec<CategoryRow>, String>>>,
    // Sales
    pub(crate) sales_docs: Vec<SalesDocRow>,
    pub(crate) new_sales_party_id: String,
    pub(crate) new_sales_desc: String,
    pub(crate) new_sales_price: String,
    pub(crate) sales_slot: Option<SharedResult<Result<Vec<SalesDocRow>, String>>>,
    // Payments
    pub(crate) payments: Vec<PaymentRow>,
    pub(crate) bank_accounts: Vec<BankAccountRow>,
    pub(crate) allocations: Vec<AllocationRow>,
    pub(crate) new_bank_name: String,
    pub(crate) new_bank_currency: String,
    pub(crate) new_pay_party_id: String,
    pub(crate) new_pay_amount: String,
    pub(crate) new_pay_ref: String,
    pub(crate) alloc_payment_id: String,
    pub(crate) alloc_document_id: String,
    pub(crate) alloc_amount: String,
    pub(crate) payments_slot: Option<SharedResult<Result<Vec<PaymentRow>, String>>>,
    pub(crate) banks_slot: Option<SharedResult<Result<Vec<BankAccountRow>, String>>>,
    pub(crate) allocs_slot: Option<SharedResult<Result<Vec<AllocationRow>, String>>>,
    // Inventory
    pub(crate) warehouses: Vec<WarehouseRow>,
    pub(crate) stock_levels: Vec<StockLevelRow>,
    pub(crate) stock_moves: Vec<StockMoveRow>,
    pub(crate) new_wh_code: String,
    pub(crate) new_wh_name: String,
    pub(crate) new_move_product_id: String,
    pub(crate) new_move_qty: String,
    pub(crate) new_move_wh_id: String,
    pub(crate) warehouses_slot: Option<SharedResult<Result<Vec<WarehouseRow>, String>>>,
    pub(crate) levels_slot: Option<SharedResult<Result<Vec<StockLevelRow>, String>>>,
    pub(crate) moves_slot: Option<SharedResult<Result<Vec<StockMoveRow>, String>>>,
    pub(crate) inventory_error: Option<String>,
    // Modules / auth
    pub(crate) modules: Vec<ModuleRow>,
    pub(crate) users: Vec<UserRow>,
    pub(crate) roles: Vec<RoleRow>,
    pub(crate) permissions: Vec<PermissionRow>,
    pub(crate) new_user_login: String,
    pub(crate) new_user_display: String,
    pub(crate) new_user_password: String,
    pub(crate) modules_slot: Option<SharedResult<Result<Vec<ModuleRow>, String>>>,
    pub(crate) users_slot: Option<SharedResult<Result<(Vec<UserRow>, Vec<RoleRow>, Vec<PermissionRow>), String>>>,
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
            new_party_name: String::new(),
            new_party_customer: true,
            new_party_supplier: false,
            new_party_prospect: false,
            form_error: None,
            mutate_slot: None,
            selected_party_id: None,
            contacts: Vec::new(),
            addresses: Vec::new(),
            new_contact_name: String::new(),
            new_contact_email: String::new(),
            new_contact_phone: String::new(),
            new_address_line1: String::new(),
            new_address_city: String::new(),
            new_address_country: "AU".into(),
            contacts_slot: None,
            addresses_slot: None,
            products: Vec::new(),
            categories: Vec::new(),
            new_product_sku: String::new(),
            new_product_name: String::new(),
            new_category_name: String::new(),
            products_slot: None,
            categories_slot: None,
            sales_docs: Vec::new(),
            new_sales_party_id: String::new(),
            new_sales_desc: String::new(),
            new_sales_price: "0".into(),
            sales_slot: None,
            payments: Vec::new(),
            bank_accounts: Vec::new(),
            allocations: Vec::new(),
            new_bank_name: String::new(),
            new_bank_currency: "AUD".into(),
            new_pay_party_id: String::new(),
            new_pay_amount: "0".into(),
            new_pay_ref: String::new(),
            alloc_payment_id: String::new(),
            alloc_document_id: String::new(),
            alloc_amount: "0".into(),
            payments_slot: None,
            banks_slot: None,
            allocs_slot: None,
            warehouses: Vec::new(),
            stock_levels: Vec::new(),
            stock_moves: Vec::new(),
            new_wh_code: String::new(),
            new_wh_name: String::new(),
            new_move_product_id: String::new(),
            new_move_qty: "1".into(),
            new_move_wh_id: String::new(),
            warehouses_slot: None,
            levels_slot: None,
            moves_slot: None,
            inventory_error: None,
            modules: Vec::new(),
            users: Vec::new(),
            roles: Vec::new(),
            permissions: Vec::new(),
            new_user_login: String::new(),
            new_user_display: String::new(),
            new_user_password: String::new(),
            modules_slot: None,
            users_slot: None,
        }
    }

    fn role_filter(&self) -> Option<PartyRole> {
        match self.nav.selected_page {
            Page::Customers => Some(PartyRole::Customer),
            Page::Suppliers => Some(PartyRole::Supplier),
            Page::Prospects => Some(PartyRole::Prospect),
            _ => None,
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
        let role_filter = self.role_filter();
        spawn_local_fut(async move {
            let snap = rusterp_api_client::refresh(&mut conn, role_filter).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(snap));
            }
        });
    }

    fn poll_detail_slots(&mut self) {
        if let Some(slot) = self.contacts_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.contacts_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.contacts = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.addresses_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.addresses_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.addresses = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
    }

    fn submit_create_party(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let name = self.new_party_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Display name is required".into());
            return;
        }
        let mut roles = Vec::new();
        if self.new_party_customer {
            roles.push(PartyRole::Customer);
        }
        if self.new_party_supplier {
            roles.push(PartyRole::Supplier);
        }
        if self.new_party_prospect {
            roles.push(PartyRole::Prospect);
        }
        if roles.is_empty() {
            self.form_error = Some("Select at least one role".into());
            return;
        }
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = create_party(&mut conn, name, roles).await.map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn request_contacts(&mut self, party_id: String) {
        if self.contacts_slot.is_some() || !live_grpc_supported() {
            return;
        }
        self.selected_party_id = Some(party_id.clone());
        let slot = shared_result();
        self.contacts_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_contacts(&mut conn, party_id).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn request_addresses(&mut self, party_id: String) {
        if self.addresses_slot.is_some() || !live_grpc_supported() {
            return;
        }
        self.selected_party_id = Some(party_id.clone());
        let slot = shared_result();
        self.addresses_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_addresses(&mut conn, party_id).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn submit_add_contact(&mut self) {
        let Some(party_id) = self.selected_party_id.clone() else {
            self.form_error = Some("Select a party first".into());
            return;
        };
        let name = self.new_contact_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Contact name is required".into());
            return;
        }
        let email = self.new_contact_email.clone();
        let phone = self.new_contact_phone.clone();
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = add_contact(&mut conn, party_id, name, email, phone)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
        self.new_contact_name.clear();
        self.new_contact_email.clear();
        self.new_contact_phone.clear();
    }

    fn submit_add_address(&mut self) {
        let Some(party_id) = self.selected_party_id.clone() else {
            self.form_error = Some("Select a party first".into());
            return;
        };
        let line1 = self.new_address_line1.trim().to_string();
        let city = self.new_address_city.trim().to_string();
        if line1.is_empty() || city.is_empty() {
            self.form_error = Some("Line1 and city are required".into());
            return;
        }
        let country = self.new_address_country.clone();
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = add_address(&mut conn, party_id, line1, city, country)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
        self.new_address_line1.clear();
        self.new_address_city.clear();
    }

    fn poll_mutate_slot(&mut self) {
        let Some(slot) = self.mutate_slot.as_ref() else {
            return;
        };
        let maybe = slot.lock().ok().and_then(|g| g.clone());
        let Some(result) = maybe else {
            return;
        };
        self.mutate_slot = None;
        match result {
            Ok(Ok(())) => {
                self.form_error = None;
                self.new_party_name.clear();
                if self.nav.selected_page.is_live_parties_list() {
                    self.request_refresh();
                } else if let Some(id) = self.selected_party_id.clone() {
                    if self.nav.selected_page.is_live_contacts() {
                        self.request_contacts(id);
                    } else if self.nav.selected_page.is_live_addresses() {
                        self.request_addresses(id);
                    }
                }
                self.reload_active_domain();
            }
            Ok(Err(msg)) | Err(msg) => {
                self.form_error = Some(msg);
            }
        }
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
                if page.is_live_parties_list() {
                    self.request_refresh();
                }
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
        ui.add_space(4.0);

        ui.collapsing("New party", |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.new_party_name);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.new_party_customer, "Customer");
                ui.checkbox(&mut self.new_party_supplier, "Supplier");
                ui.checkbox(&mut self.new_party_prospect, "Prospect");
            });
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create").fill(tokens::ACCENT),
                )
                .clicked()
            {
                self.submit_create_party();
            }
        });
        if let Some(err) = &self.form_error {
            ui.colored_label(tokens::ERROR, err);
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

    fn ensure_parties_loaded(&mut self) {
        if self.parties.is_empty()
            && live_grpc_supported()
            && !self.refresh_in_flight()
            && self.status != ConnectionStatus::Connecting
        {
            self.request_refresh();
        }
    }

    fn draw_contacts_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Contacts");
        self.ensure_parties_loaded();
        ui.add_space(4.0);
        ui.label("Party");
        egui::ComboBox::from_id_salt("contact_party")
            .selected_text(
                self.selected_party_id
                    .as_ref()
                    .and_then(|id| {
                        self.parties
                            .iter()
                            .find(|p| &p.id == id)
                            .map(|p| p.display_name.as_str())
                    })
                    .unwrap_or("Select party…"),
            )
            .show_ui(ui, |ui| {
                for p in self.parties.clone() {
                    if ui
                        .selectable_label(
                            self.selected_party_id.as_deref() == Some(p.id.as_str()),
                            &p.display_name,
                        )
                        .clicked()
                    {
                        self.request_contacts(p.id);
                    }
                }
            });

        ui.add_space(8.0);
        ui.collapsing("Add contact", |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.new_contact_name);
            });
            ui.horizontal(|ui| {
                ui.label("Email");
                ui.text_edit_singleline(&mut self.new_contact_email);
            });
            ui.horizontal(|ui| {
                ui.label("Phone");
                ui.text_edit_singleline(&mut self.new_contact_phone);
            });
            if ui.button("Add").clicked() {
                self.submit_add_contact();
            }
        });
        if let Some(err) = &self.form_error {
            ui.colored_label(tokens::ERROR, err);
        }
        ui.add_space(8.0);
        if self.selected_party_id.is_none() {
            ui.label("Select a party to list contacts.");
            return;
        }
        if self.contacts.is_empty() {
            ui.label("No contacts for this party.");
            return;
        }
        egui::Grid::new("contacts_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("Name");
                ui.strong("Email");
                ui.strong("Phone");
                ui.strong("Id");
                ui.end_row();
                for c in &self.contacts {
                    ui.label(&c.name);
                    ui.label(&c.email);
                    ui.label(&c.phone);
                    ui.monospace(&c.id);
                    ui.end_row();
                }
            });
    }

    fn draw_addresses_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Addresses");
        self.ensure_parties_loaded();
        ui.add_space(4.0);
        ui.label("Party");
        egui::ComboBox::from_id_salt("address_party")
            .selected_text(
                self.selected_party_id
                    .as_ref()
                    .and_then(|id| {
                        self.parties
                            .iter()
                            .find(|p| &p.id == id)
                            .map(|p| p.display_name.as_str())
                    })
                    .unwrap_or("Select party…"),
            )
            .show_ui(ui, |ui| {
                for p in self.parties.clone() {
                    if ui
                        .selectable_label(
                            self.selected_party_id.as_deref() == Some(p.id.as_str()),
                            &p.display_name,
                        )
                        .clicked()
                    {
                        self.request_addresses(p.id);
                    }
                }
            });

        ui.add_space(8.0);
        ui.collapsing("Add address (billing)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Line 1");
                ui.text_edit_singleline(&mut self.new_address_line1);
            });
            ui.horizontal(|ui| {
                ui.label("City");
                ui.text_edit_singleline(&mut self.new_address_city);
            });
            ui.horizontal(|ui| {
                ui.label("Country");
                ui.text_edit_singleline(&mut self.new_address_country);
            });
            if ui.button("Add").clicked() {
                self.submit_add_address();
            }
        });
        if let Some(err) = &self.form_error {
            ui.colored_label(tokens::ERROR, err);
        }
        ui.add_space(8.0);
        if self.selected_party_id.is_none() {
            ui.label("Select a party to list addresses.");
            return;
        }
        if self.addresses.is_empty() {
            ui.label("No addresses for this party.");
            return;
        }
        egui::Grid::new("addresses_grid")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("Kind");
                ui.strong("Line 1");
                ui.strong("City");
                ui.strong("Country");
                ui.strong("Id");
                ui.end_row();
                for a in &self.addresses {
                    ui.label(&a.kind);
                    ui.label(&a.line1);
                    ui.label(&a.city);
                    ui.label(&a.country);
                    ui.monospace(&a.id);
                    ui.end_row();
                }
            });
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
        self.draw_live_modules(ui);
    }

    fn draw_settings_users_roles(&mut self, ui: &mut egui::Ui) {
        self.draw_live_users_roles(ui);
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
            "https://github.com/ndx-au/RustERP",
        );
        ui.hyperlink_to("RustERP.biz", "https://RustERP.biz");
    }

    fn draw_content(&mut self, ui: &mut egui::Ui) {
        ui.set_width(ui.available_width());
        match self.nav.selected_page {
            Page::AllParties | Page::Customers | Page::Suppliers | Page::Prospects => {
                self.draw_parties_content(ui);
            }
            Page::Contacts => self.draw_contacts_content(ui),
            Page::Addresses => self.draw_addresses_content(ui),
            Page::Products => self.draw_products_content(ui),
            Page::Categories => self.draw_categories_content(ui),
            Page::Quotes | Page::Orders | Page::Invoices | Page::CreditNotes => {
                self.draw_sales_content(ui);
            }
            Page::PaymentsList => self.draw_payments_content(ui),
            Page::BankAccounts => self.draw_bank_accounts_content(ui),
            Page::Allocations => self.draw_allocations_content(ui),
            Page::Warehouses => self.draw_warehouses_content(ui),
            Page::StockLevels => self.draw_stock_levels_content(ui),
            Page::StockMoves => self.draw_stock_moves_content(ui),
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
        self.poll_mutate_slot();
        self.poll_detail_slots();
        self.poll_domain_slots();

        if self.refresh_in_flight()
            || self.mutate_slot.is_some()
            || self.contacts_slot.is_some()
            || self.addresses_slot.is_some()
            || self.domain_slots_busy()
        {
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
