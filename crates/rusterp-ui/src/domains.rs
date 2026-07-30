//! Live domain pages (catalog / sales / payments / inventory / modules / auth).

use crate::app::ReferenceApp;
use crate::shell::{tokens, Page, SettingsTab};
use rusterp_api_client::{
    create_allocation, create_bank_account, create_category, create_payment, create_product,
    create_sales_document, create_stock_move, create_user, create_warehouse, list_allocations,
    list_bank_accounts, list_categories, list_modules, list_payments, list_permissions,
    list_products, list_roles, list_sales_documents, list_stock_levels, list_stock_moves,
    list_users, list_warehouses, live_grpc_supported, set_module_enabled, shared_result,
    spawn_local_fut, Connection, DocumentKind,
};

impl ReferenceApp {
    pub(crate) fn domain_slots_busy(&self) -> bool {
        self.products_slot.is_some()
            || self.categories_slot.is_some()
            || self.sales_slot.is_some()
            || self.payments_slot.is_some()
            || self.banks_slot.is_some()
            || self.allocs_slot.is_some()
            || self.warehouses_slot.is_some()
            || self.levels_slot.is_some()
            || self.moves_slot.is_some()
            || self.modules_slot.is_some()
            || self.users_slot.is_some()
    }

    pub(crate) fn poll_domain_slots(&mut self) {
        if let Some(slot) = self.products_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.products_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.products = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.categories_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.categories_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.categories = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.sales_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.sales_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.sales_docs = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.payments_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.payments_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.payments = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.banks_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.banks_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.bank_accounts = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.allocs_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.allocs_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.allocations = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.warehouses_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.warehouses_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.warehouses = rows;
                        self.inventory_error = None;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => {
                        self.inventory_error = Some(msg.clone());
                        self.form_error = Some(msg);
                    }
                }
            }
        }
        if let Some(slot) = self.levels_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.levels_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.stock_levels = rows;
                        self.inventory_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.inventory_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.moves_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.moves_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.stock_moves = rows;
                        self.inventory_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.inventory_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.modules_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.modules_slot = None;
                match result {
                    Ok(Ok(rows)) => {
                        self.modules = rows;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
        if let Some(slot) = self.users_slot.as_ref() {
            if let Some(result) = slot.lock().ok().and_then(|g| g.clone()) {
                self.users_slot = None;
                match result {
                    Ok(Ok((users, roles, perms))) => {
                        self.users = users;
                        self.roles = roles;
                        self.permissions = perms;
                        self.form_error = None;
                    }
                    Ok(Err(msg)) | Err(msg) => self.form_error = Some(msg),
                }
            }
        }
    }

    pub(crate) fn reload_active_domain(&mut self) {
        match self.nav.selected_page {
            Page::Products => self.request_products(),
            Page::Categories => self.request_categories(),
            Page::Quotes | Page::Orders | Page::Invoices | Page::CreditNotes => {
                self.request_sales()
            }
            Page::PaymentsList => self.request_payments(),
            Page::BankAccounts => self.request_banks(),
            Page::Allocations => {
                if !self.alloc_payment_id.trim().is_empty() {
                    self.request_allocations();
                }
            }
            Page::Warehouses => self.request_warehouses(),
            Page::StockLevels => self.request_stock_levels(),
            Page::StockMoves => self.request_stock_moves(),
            Page::SettingsHost => match self.nav.selected_tab {
                SettingsTab::Modules => self.request_modules(),
                SettingsTab::UsersAndRoles => self.request_users_roles(),
                _ => {}
            },
            _ => {}
        }
    }

    fn sales_kind(&self) -> DocumentKind {
        match self.nav.selected_page {
            Page::Orders => DocumentKind::Order,
            Page::Invoices => DocumentKind::Invoice,
            Page::CreditNotes => DocumentKind::CreditNote,
            _ => DocumentKind::Quote,
        }
    }

    pub(crate) fn request_products(&mut self) {
        if self.products_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.products_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_products(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_categories(&mut self) {
        if self.categories_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.categories_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_categories(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_sales(&mut self) {
        if self.sales_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let kind = self.sales_kind();
        let slot = shared_result();
        self.sales_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_sales_documents(&mut conn, kind).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_payments(&mut self) {
        if self.payments_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.payments_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_payments(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_banks(&mut self) {
        if self.banks_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.banks_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_bank_accounts(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_allocations(&mut self) {
        if self.allocs_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let payment_id = self.alloc_payment_id.trim().to_string();
        if payment_id.is_empty() {
            return;
        }
        let slot = shared_result();
        self.allocs_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_allocations(&mut conn, payment_id).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_warehouses(&mut self) {
        if self.warehouses_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.warehouses_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_warehouses(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_stock_levels(&mut self) {
        if self.levels_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.levels_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_stock_levels(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_stock_moves(&mut self) {
        if self.moves_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.moves_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_stock_moves(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_modules(&mut self) {
        if self.modules_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.modules_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = list_modules(&mut conn).await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn request_users_roles(&mut self) {
        if self.users_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let slot = shared_result();
        self.users_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = async {
                let users = list_users(&mut conn).await?;
                let roles = list_roles(&mut conn).await?;
                let permissions = list_permissions(&mut conn).await?;
                Ok((users, roles, permissions))
            }
            .await;
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    pub(crate) fn draw_products_content(&mut self, ui: &mut egui::Ui) {
        if self.products.is_empty() && self.products_slot.is_none() && live_grpc_supported() {
            self.request_products();
        }
        ui.heading("Products");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("SKU");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_product_sku)
                    .desired_width(100.0)
                    .hint_text("SKU"),
            );
            ui.label("Name");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_product_name)
                    .desired_width(160.0)
                    .hint_text("Name"),
            );
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let sku = self.new_product_sku.trim().to_string();
                let name = self.new_product_name.trim().to_string();
                if sku.is_empty() || name.is_empty() {
                    self.form_error = Some("SKU and name required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_product(&mut conn, sku, name).await.map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_product_sku.clear();
                    self.new_product_name.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_products();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("products_grid")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("sku");
                ui.strong("name");
                ui.strong("type");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for p in &self.products {
                    ui.monospace(&p.sku);
                    ui.label(&p.name);
                    ui.label(&p.type_label);
                    ui.label(if p.active { "yes" } else { "no" });
                    ui.monospace(&p.id);
                    ui.end_row();
                }
            });
        if self.products.is_empty() && self.products_slot.is_none() {
            ui.label(egui::RichText::new("No products.").weak());
        }
    }

    pub(crate) fn draw_categories_content(&mut self, ui: &mut egui::Ui) {
        if self.categories.is_empty() && self.categories_slot.is_none() && live_grpc_supported() {
            self.request_categories();
        }
        ui.heading("Categories");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_category_name)
                    .desired_width(180.0)
                    .hint_text("Category name"),
            );
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let name = self.new_category_name.trim().to_string();
                if name.is_empty() {
                    self.form_error = Some("Name required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_category(&mut conn, name).await.map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_category_name.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_categories();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("categories_grid")
            .striped(true)
            .num_columns(3)
            .show(ui, |ui| {
                ui.strong("name");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for c in &self.categories {
                    ui.label(&c.name);
                    ui.label(if c.active { "yes" } else { "no" });
                    ui.monospace(&c.id);
                    ui.end_row();
                }
            });
        if self.categories.is_empty() && self.categories_slot.is_none() {
            ui.label(egui::RichText::new("No categories.").weak());
        }
    }

    pub(crate) fn draw_sales_content(&mut self, ui: &mut egui::Ui) {
        if self.sales_docs.is_empty() && self.sales_slot.is_none() && live_grpc_supported() {
            self.request_sales();
        }
        ui.heading(self.nav.selected_page.label());
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Party id");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_sales_party_id)
                    .desired_width(200.0)
                    .hint_text("party uuid"),
            );
            ui.label("Desc");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_sales_desc)
                    .desired_width(140.0)
                    .hint_text("line description"),
            );
            ui.label("Price (minor)");
            ui.add(
                egui::TextEdit::singleline(&mut self.new_sales_price)
                    .desired_width(80.0)
                    .hint_text("cents"),
            );
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create draft").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let party_id = self.new_sales_party_id.trim().to_string();
                let description = self.new_sales_desc.trim().to_string();
                let price: i64 = self.new_sales_price.trim().parse().unwrap_or(-1);
                if party_id.is_empty() || description.is_empty() || price < 0 {
                    self.form_error = Some("Party, description, and non-negative price required".into());
                } else {
                    let kind = self.sales_kind();
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result =
                            create_sales_document(&mut conn, kind, party_id, description, price)
                                .await
                                .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_sales_desc.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_sales();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("sales_grid")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.strong("number");
                ui.strong("status");
                ui.strong("party");
                ui.strong("total");
                ui.strong("id");
                ui.strong("");
                ui.end_row();
                for d in &self.sales_docs {
                    ui.monospace(&d.number);
                    ui.label(&d.status);
                    ui.monospace(&d.party_id);
                    ui.label(d.total_minor.to_string());
                    ui.monospace(&d.id);
                    ui.label("");
                    ui.end_row();
                }
            });
        if self.sales_docs.is_empty() && self.sales_slot.is_none() {
            ui.label(egui::RichText::new("No documents.").weak());
        }
    }

    pub(crate) fn draw_payments_content(&mut self, ui: &mut egui::Ui) {
        if self.payments.is_empty() && self.payments_slot.is_none() && live_grpc_supported() {
            self.request_payments();
        }
        ui.heading("Payments");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Party");
            ui.add(egui::TextEdit::singleline(&mut self.new_pay_party_id).desired_width(180.0));
            ui.label("Amount");
            ui.add(egui::TextEdit::singleline(&mut self.new_pay_amount).desired_width(80.0));
            ui.label("Ref");
            ui.add(egui::TextEdit::singleline(&mut self.new_pay_ref).desired_width(100.0));
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create inbound").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let party_id = self.new_pay_party_id.trim().to_string();
                let amount: i64 = self.new_pay_amount.trim().parse().unwrap_or(-1);
                let reference = self.new_pay_ref.clone();
                if party_id.is_empty() || amount < 0 {
                    self.form_error = Some("Party and non-negative amount required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result =
                            create_payment(&mut conn, party_id, amount, "AUD".into(), reference)
                                .await
                                .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_payments();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("payments_grid")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.strong("dir");
                ui.strong("amount");
                ui.strong("party");
                ui.strong("ref");
                ui.strong("status");
                ui.strong("id");
                ui.end_row();
                for p in &self.payments {
                    ui.label(&p.direction);
                    ui.label(format!("{} {}", p.amount_minor, p.currency));
                    ui.monospace(&p.party_id);
                    ui.label(&p.reference);
                    ui.label(&p.status);
                    ui.monospace(&p.id);
                    ui.end_row();
                }
            });
        if self.payments.is_empty() && self.payments_slot.is_none() {
            ui.label(egui::RichText::new("No payments.").weak());
        }
    }

    pub(crate) fn draw_bank_accounts_content(&mut self, ui: &mut egui::Ui) {
        if self.bank_accounts.is_empty() && self.banks_slot.is_none() && live_grpc_supported() {
            self.request_banks();
        }
        ui.heading("Bank Accounts");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.add(egui::TextEdit::singleline(&mut self.new_bank_name).desired_width(160.0));
            ui.label("Currency");
            ui.add(egui::TextEdit::singleline(&mut self.new_bank_currency).desired_width(60.0));
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let name = self.new_bank_name.trim().to_string();
                let currency = self.new_bank_currency.trim().to_string();
                if name.is_empty() || currency.len() != 3 {
                    self.form_error = Some("Name and 3-letter currency required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_bank_account(&mut conn, name, currency)
                            .await
                            .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_bank_name.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_banks();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("banks_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("name");
                ui.strong("currency");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for a in &self.bank_accounts {
                    ui.label(&a.name);
                    ui.label(&a.currency);
                    ui.label(if a.active { "yes" } else { "no" });
                    ui.monospace(&a.id);
                    ui.end_row();
                }
            });
        if self.bank_accounts.is_empty() && self.banks_slot.is_none() {
            ui.label(egui::RichText::new("No bank accounts.").weak());
        }
    }

    pub(crate) fn draw_allocations_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Allocations");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Payment id");
            ui.add(egui::TextEdit::singleline(&mut self.alloc_payment_id).desired_width(200.0));
            ui.label("Document id");
            ui.add(egui::TextEdit::singleline(&mut self.alloc_document_id).desired_width(200.0));
            ui.label("Amount");
            ui.add(egui::TextEdit::singleline(&mut self.alloc_amount).desired_width(80.0));
            if ui.button("Load").clicked() {
                self.request_allocations();
            }
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Allocate").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let payment_id = self.alloc_payment_id.trim().to_string();
                let document_id = self.alloc_document_id.trim().to_string();
                let amount: i64 = self.alloc_amount.trim().parse().unwrap_or(-1);
                if payment_id.is_empty() || document_id.is_empty() || amount <= 0 {
                    self.form_error = Some("Payment, document, and positive amount required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result =
                            create_allocation(&mut conn, payment_id, document_id, amount)
                                .await
                                .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                }
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("allocs_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("payment");
                ui.strong("document");
                ui.strong("amount");
                ui.strong("id");
                ui.end_row();
                for a in &self.allocations {
                    ui.monospace(&a.payment_id);
                    ui.monospace(&a.document_id);
                    ui.label(a.amount_minor.to_string());
                    ui.monospace(&a.id);
                    ui.end_row();
                }
            });
        if self.allocations.is_empty() {
            ui.label(egui::RichText::new("No allocations loaded.").weak());
        }
    }

    pub(crate) fn draw_warehouses_content(&mut self, ui: &mut egui::Ui) {
        if self.warehouses.is_empty()
            && self.warehouses_slot.is_none()
            && self.inventory_error.is_none()
            && live_grpc_supported()
        {
            self.request_warehouses();
        }
        ui.heading("Warehouses");
        ui.add_space(4.0);
        if let Some(err) = &self.inventory_error {
            ui.colored_label(
                tokens::ERROR,
                format!("{err} — enable Inventory under Settings → Modules"),
            );
            ui.add_space(4.0);
        }
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Code");
            ui.add(egui::TextEdit::singleline(&mut self.new_wh_code).desired_width(80.0));
            ui.label("Name");
            ui.add(egui::TextEdit::singleline(&mut self.new_wh_name).desired_width(160.0));
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let code = self.new_wh_code.trim().to_string();
                let name = self.new_wh_name.trim().to_string();
                if code.is_empty() || name.is_empty() {
                    self.form_error = Some("Code and name required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_warehouse(&mut conn, code, name).await.map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_wh_code.clear();
                    self.new_wh_name.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.inventory_error = None;
                self.request_warehouses();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("wh_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("code");
                ui.strong("name");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for w in &self.warehouses {
                    ui.monospace(&w.code);
                    ui.label(&w.name);
                    ui.label(if w.active { "yes" } else { "no" });
                    ui.monospace(&w.id);
                    ui.end_row();
                }
            });
        if self.warehouses.is_empty() && self.warehouses_slot.is_none() && self.inventory_error.is_none()
        {
            ui.label(egui::RichText::new("No warehouses.").weak());
        }
    }

    pub(crate) fn draw_stock_levels_content(&mut self, ui: &mut egui::Ui) {
        if self.stock_levels.is_empty()
            && self.levels_slot.is_none()
            && self.inventory_error.is_none()
            && live_grpc_supported()
        {
            self.request_stock_levels();
        }
        ui.heading("Stock Levels");
        ui.add_space(4.0);
        if let Some(err) = &self.inventory_error {
            ui.colored_label(tokens::ERROR, err);
        }
        if ui.button("Refresh").clicked() {
            self.inventory_error = None;
            self.request_stock_levels();
        }
        ui.add_space(8.0);
        egui::Grid::new("levels_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("warehouse");
                ui.strong("product");
                ui.strong("on hand");
                ui.strong("reserved");
                ui.end_row();
                for l in &self.stock_levels {
                    ui.monospace(&l.warehouse_id);
                    ui.monospace(&l.product_id);
                    ui.label(&l.qty_on_hand);
                    ui.label(&l.qty_reserved);
                    ui.end_row();
                }
            });
        if self.stock_levels.is_empty() && self.levels_slot.is_none() && self.inventory_error.is_none()
        {
            ui.label(egui::RichText::new("No stock levels.").weak());
        }
    }

    pub(crate) fn draw_stock_moves_content(&mut self, ui: &mut egui::Ui) {
        if self.stock_moves.is_empty()
            && self.moves_slot.is_none()
            && self.inventory_error.is_none()
            && live_grpc_supported()
        {
            self.request_stock_moves();
        }
        ui.heading("Stock Moves");
        ui.add_space(4.0);
        if let Some(err) = &self.inventory_error {
            ui.colored_label(tokens::ERROR, err);
        }
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Product");
            ui.add(egui::TextEdit::singleline(&mut self.new_move_product_id).desired_width(180.0));
            ui.label("Qty");
            ui.add(egui::TextEdit::singleline(&mut self.new_move_qty).desired_width(60.0));
            ui.label("To warehouse");
            ui.add(egui::TextEdit::singleline(&mut self.new_move_wh_id).desired_width(180.0));
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create move").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let product_id = self.new_move_product_id.trim().to_string();
                let qty = self.new_move_qty.trim().to_string();
                let wh = self.new_move_wh_id.trim().to_string();
                if product_id.is_empty() || qty.is_empty() || wh.is_empty() {
                    self.form_error = Some("Product, qty, and warehouse required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_stock_move(&mut conn, product_id, qty, wh)
                            .await
                            .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                }
            }
            if ui.button("Refresh").clicked() {
                self.inventory_error = None;
                self.request_stock_moves();
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("moves_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("product");
                ui.strong("qty");
                ui.strong("state");
                ui.strong("id");
                ui.end_row();
                for m in &self.stock_moves {
                    ui.monospace(&m.product_id);
                    ui.label(&m.qty);
                    ui.label(&m.state);
                    ui.monospace(&m.id);
                    ui.end_row();
                }
            });
        if self.stock_moves.is_empty() && self.moves_slot.is_none() && self.inventory_error.is_none()
        {
            ui.label(egui::RichText::new("No stock moves.").weak());
        }
    }

    pub(crate) fn draw_live_modules(&mut self, ui: &mut egui::Ui) {
        if self.modules.is_empty() && self.modules_slot.is_none() && live_grpc_supported() {
            self.request_modules();
        }
        ui.label(egui::RichText::new("Modules").strong());
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Live from core.modules — toggle inventory when ready.")
                .small()
                .weak(),
        );
        self.draw_form_error(ui);
        if ui.button("Refresh").clicked() {
            self.request_modules();
        }
        ui.add_space(8.0);
        egui::Grid::new("modules_live_grid")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("id");
                ui.strong("name");
                ui.strong("enabled");
                ui.strong("always_on");
                ui.strong("");
                ui.end_row();
                let rows: Vec<_> = self.modules.clone();
                for row in rows {
                    ui.monospace(&row.id);
                    ui.label(&row.name);
                    ui.label(if row.enabled { "yes" } else { "no" });
                    ui.label(if row.always_on { "yes" } else { "no" });
                    if row.always_on {
                        ui.label("—");
                    } else if ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new(if row.enabled { "Disable" } else { "Enable" }),
                        )
                        .clicked()
                    {
                        let id = row.id.clone();
                        let enabled = !row.enabled;
                        let slot = shared_result();
                        self.mutate_slot = Some(slot.clone());
                        let url = self.rpc_url.clone();
                        spawn_local_fut(async move {
                            let mut conn = Connection::new(url);
                            let result = set_module_enabled(&mut conn, id, enabled)
                                .await
                                .map(|_| ());
                            if let Ok(mut g) = slot.lock() {
                                *g = Some(Ok(result));
                            }
                        });
                    }
                    ui.end_row();
                }
            });
        if self.modules.is_empty() && self.modules_slot.is_none() {
            ui.label(egui::RichText::new("No modules loaded.").weak());
        }
    }

    pub(crate) fn draw_live_users_roles(&mut self, ui: &mut egui::Ui) {
        if self.users.is_empty() && self.users_slot.is_none() && live_grpc_supported() {
            self.request_users_roles();
        }
        ui.label(egui::RichText::new("Users & Roles").strong());
        ui.add_space(4.0);
        self.draw_form_error(ui);
        ui.horizontal(|ui| {
            ui.label("Login");
            ui.add(egui::TextEdit::singleline(&mut self.new_user_login).desired_width(100.0));
            ui.label("Display");
            ui.add(egui::TextEdit::singleline(&mut self.new_user_display).desired_width(120.0));
            ui.label("Password");
            ui.add(egui::TextEdit::singleline(&mut self.new_user_password).desired_width(100.0));
            if ui
                .add_enabled(
                    live_grpc_supported() && self.mutate_slot.is_none(),
                    egui::Button::new("Create user").fill(tokens::ACCENT),
                )
                .clicked()
            {
                let login = self.new_user_login.trim().to_string();
                let display = self.new_user_display.trim().to_string();
                let password = self.new_user_password.clone();
                if login.is_empty() || display.is_empty() || password.is_empty() {
                    self.form_error = Some("Login, display name, and password required".into());
                } else {
                    let slot = shared_result();
                    self.mutate_slot = Some(slot.clone());
                    let url = self.rpc_url.clone();
                    spawn_local_fut(async move {
                        let mut conn = Connection::new(url);
                        let result = create_user(&mut conn, login, display, password)
                            .await
                            .map(|_| ());
                        if let Ok(mut g) = slot.lock() {
                            *g = Some(Ok(result));
                        }
                    });
                    self.new_user_login.clear();
                    self.new_user_display.clear();
                    self.new_user_password.clear();
                }
            }
            if ui.button("Refresh").clicked() {
                self.request_users_roles();
            }
        });
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Users").strong());
        egui::Grid::new("users_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("login");
                ui.strong("display");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for u in &self.users {
                    ui.monospace(&u.login);
                    ui.label(&u.display_name);
                    ui.label(if u.active { "yes" } else { "no" });
                    ui.monospace(&u.id);
                    ui.end_row();
                }
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Roles").strong());
        egui::Grid::new("roles_grid")
            .striped(true)
            .num_columns(3)
            .show(ui, |ui| {
                ui.strong("name");
                ui.strong("description");
                ui.strong("id");
                ui.end_row();
                for r in &self.roles {
                    ui.label(&r.name);
                    ui.label(&r.description);
                    ui.monospace(&r.id);
                    ui.end_row();
                }
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Permissions").strong());
        egui::Grid::new("perms_grid")
            .striped(true)
            .num_columns(3)
            .show(ui, |ui| {
                ui.strong("resource");
                ui.strong("action");
                ui.strong("id");
                ui.end_row();
                for p in &self.permissions {
                    ui.monospace(&p.resource);
                    ui.monospace(&p.action);
                    ui.monospace(&p.id);
                    ui.end_row();
                }
            });
    }

    fn draw_form_error(&self, ui: &mut egui::Ui) {
        if let Some(err) = &self.form_error {
            ui.colored_label(tokens::ERROR, err);
            ui.add_space(4.0);
        }
    }
}
