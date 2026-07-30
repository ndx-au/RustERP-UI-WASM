//! Live domain pages (catalog / sales / payments / inventory / modules / auth).

use crate::app::ReferenceApp;
use crate::forms::{focus_once, form_keys, text_field};
use crate::shell::{tokens, Page, SettingsTab};
use rusterp_api_client::{
    create_allocation, create_bank_account, create_category, create_payment, create_product,
    create_sales_document, create_stock_move, create_user, create_warehouse, list_allocations,
    list_bank_accounts, list_categories, list_modules, list_payments, list_permissions,
    list_products, list_roles, list_sales_documents, list_stock_levels, list_stock_moves,
    list_users, list_warehouses, live_grpc_supported, set_module_enabled,
    set_sales_document_status, shared_result, spawn_local_fut, update_bank_account,
    update_category, update_payment, update_product, update_sales_document, update_user,
    update_warehouse, BankAccountRow, CategoryRow, Connection, DocumentKind, DocumentStatus,
    PaymentRow, ProductRow, SalesDocRow, UserRow, WarehouseRow,
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
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("SKU");
            let id = egui::Id::new("create_product_sku");
            focus_once(ui, id, &mut self.focus_create_product);
            text_field(ui, &mut self.new_product_sku, 100.0, "SKU", id, &mut create_focused);
            ui.label("Name");
            let id = egui::Id::new("create_product_name");
            text_field(ui, &mut self.new_product_name, 160.0, "Name", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_product_sku.clear();
                self.new_product_name.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_products();
            }
        });
        ui.add_space(8.0);
        let products: Vec<_> = self.products.clone();
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
                for p in &products {
                    let selected = self.selected_edit_product_id.as_deref() == Some(p.id.as_str());
                    ui.monospace(&p.sku);
                    if ui.selectable_label(selected, &p.name).clicked() {
                        self.select_product_for_edit(p);
                    }
                    ui.label(&p.type_label);
                    ui.label(if p.active { "yes" } else { "no" });
                    ui.monospace(&p.id);
                    ui.end_row();
                }
            });
        if self.products.is_empty() && self.products_slot.is_none() {
            ui.label(egui::RichText::new("No products.").weak());
        }
        if self.selected_edit_product_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit product").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("SKU");
                let id = egui::Id::new("edit_product_sku");
                focus_once(ui, id, &mut self.focus_edit_product);
                text_field(ui, &mut self.edit_product_sku, 100.0, "SKU", id, &mut focused);
                ui.label("Name");
                let id = egui::Id::new("edit_product_name");
                text_field(ui, &mut self.edit_product_name, 160.0, "Name", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_product_active, "Active");
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_product();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
    }

    pub(crate) fn draw_categories_content(&mut self, ui: &mut egui::Ui) {
        if self.categories.is_empty() && self.categories_slot.is_none() && live_grpc_supported() {
            self.request_categories();
        }
        ui.heading("Categories");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Name");
            let id = egui::Id::new("create_category_name");
            focus_once(ui, id, &mut self.focus_create_category);
            text_field(
                ui,
                &mut self.new_category_name,
                180.0,
                "Category name",
                id,
                &mut create_focused,
            );
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_category_name.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_categories();
            }
        });
        ui.add_space(8.0);
        let categories: Vec<_> = self.categories.clone();
        egui::Grid::new("categories_grid")
            .striped(true)
            .num_columns(3)
            .show(ui, |ui| {
                ui.strong("name");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for c in &categories {
                    let selected = self.selected_edit_category_id.as_deref() == Some(c.id.as_str());
                    if ui.selectable_label(selected, &c.name).clicked() {
                        self.select_category_for_edit(c);
                    }
                    ui.label(if c.active { "yes" } else { "no" });
                    ui.monospace(&c.id);
                    ui.end_row();
                }
            });
        if self.categories.is_empty() && self.categories_slot.is_none() {
            ui.label(egui::RichText::new("No categories.").weak());
        }
        if self.selected_edit_category_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit category").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Name");
                let id = egui::Id::new("edit_category_name");
                focus_once(ui, id, &mut self.focus_edit_category);
                text_field(ui, &mut self.edit_category_name, 180.0, "Name", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_category_active, "Active");
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_category();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
    }

    pub(crate) fn draw_sales_content(&mut self, ui: &mut egui::Ui) {
        if self.sales_docs.is_empty() && self.sales_slot.is_none() && live_grpc_supported() {
            self.request_sales();
        }
        ui.heading(self.nav.selected_page.label());
        ui.add_space(4.0);
        self.draw_form_error(ui);
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Party id");
            let id = egui::Id::new("create_sales_party");
            focus_once(ui, id, &mut self.focus_create_sales);
            text_field(
                ui,
                &mut self.new_sales_party_id,
                200.0,
                "party uuid",
                id,
                &mut create_focused,
            );
            ui.label("Desc");
            let id = egui::Id::new("create_sales_desc");
            text_field(
                ui,
                &mut self.new_sales_desc,
                140.0,
                "line description",
                id,
                &mut create_focused,
            );
            ui.label("Price (minor)");
            let id = egui::Id::new("create_sales_price");
            text_field(ui, &mut self.new_sales_price, 80.0, "cents", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create draft").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
            {
                let party_id = self.new_sales_party_id.trim().to_string();
                let description = self.new_sales_desc.trim().to_string();
                let price: i64 = self.new_sales_price.trim().parse().unwrap_or(-1);
                if party_id.is_empty() || description.is_empty() || price < 0 {
                    self.form_error =
                        Some("Party, description, and non-negative price required".into());
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
            if create_keys.cancel {
                self.new_sales_desc.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_sales();
            }
        });
        ui.add_space(8.0);
        let sales_docs: Vec<_> = self.sales_docs.clone();
        egui::Grid::new("sales_grid")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.strong("number");
                ui.strong("status");
                ui.strong("party");
                ui.strong("total");
                ui.strong("notes");
                ui.strong("id");
                ui.end_row();
                for d in &sales_docs {
                    let selected = self.selected_edit_sales_id.as_deref() == Some(d.id.as_str());
                    if ui.selectable_label(selected, &d.number).clicked() {
                        self.select_sales_for_edit(d);
                    }
                    ui.label(&d.status);
                    ui.monospace(&d.party_id);
                    ui.label(d.total_minor.to_string());
                    ui.label(if d.notes.is_empty() { "—" } else { &d.notes });
                    ui.monospace(&d.id);
                    ui.end_row();
                }
            });
        if self.sales_docs.is_empty() && self.sales_slot.is_none() {
            ui.label(egui::RichText::new("No documents.").weak());
        }
        if self.selected_edit_sales_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit document").strong());
            ui.label(format!("Status: {}", self.edit_sales_status));
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Notes");
                let id = egui::Id::new("edit_sales_notes");
                focus_once(ui, id, &mut self.focus_edit_sales);
                text_field(ui, &mut self.edit_sales_notes, 300.0, "Notes", id, &mut focused);
            });
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save notes").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_sales();
                }
                if self.edit_sales_status == "draft"
                    && ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Confirm"),
                        )
                        .clicked()
                {
                    self.submit_sales_status(DocumentStatus::Confirmed);
                }
                if self.edit_sales_status == "confirmed"
                    && ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Post"),
                        )
                        .clicked()
                {
                    self.submit_sales_status(DocumentStatus::Posted);
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
    }

    pub(crate) fn draw_payments_content(&mut self, ui: &mut egui::Ui) {
        if self.payments.is_empty() && self.payments_slot.is_none() && live_grpc_supported() {
            self.request_payments();
        }
        ui.heading("Payments");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Party");
            let id = egui::Id::new("create_pay_party");
            focus_once(ui, id, &mut self.focus_create_payment);
            text_field(ui, &mut self.new_pay_party_id, 180.0, "party uuid", id, &mut create_focused);
            ui.label("Amount");
            let id = egui::Id::new("create_pay_amount");
            text_field(ui, &mut self.new_pay_amount, 80.0, "cents", id, &mut create_focused);
            ui.label("Ref");
            let id = egui::Id::new("create_pay_ref");
            text_field(ui, &mut self.new_pay_ref, 100.0, "reference", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create inbound").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_pay_ref.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_payments();
            }
        });
        ui.add_space(8.0);
        let payments: Vec<_> = self.payments.clone();
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
                for p in &payments {
                    ui.label(&p.direction);
                    ui.label(format!("{} {}", p.amount_minor, p.currency));
                    ui.monospace(&p.party_id);
                    let selected = self.selected_edit_payment_id.as_deref() == Some(p.id.as_str());
                    if ui.selectable_label(selected, &p.reference).clicked() {
                        self.select_payment_for_edit(p);
                    }
                    ui.label(&p.status);
                    ui.monospace(&p.id);
                    ui.end_row();
                }
            });
        if self.payments.is_empty() && self.payments_slot.is_none() {
            ui.label(egui::RichText::new("No payments.").weak());
        }
        if self.selected_edit_payment_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit payment").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Reference");
                let id = egui::Id::new("edit_payment_ref");
                focus_once(ui, id, &mut self.focus_edit_payment);
                text_field(ui, &mut self.edit_payment_ref, 200.0, "Reference", id, &mut focused);
            });
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_payment();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
    }

    pub(crate) fn draw_bank_accounts_content(&mut self, ui: &mut egui::Ui) {
        if self.bank_accounts.is_empty() && self.banks_slot.is_none() && live_grpc_supported() {
            self.request_banks();
        }
        ui.heading("Bank Accounts");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Name");
            let id = egui::Id::new("create_bank_name");
            focus_once(ui, id, &mut self.focus_create_bank);
            text_field(ui, &mut self.new_bank_name, 160.0, "Name", id, &mut create_focused);
            ui.label("Currency");
            let id = egui::Id::new("create_bank_currency");
            text_field(ui, &mut self.new_bank_currency, 60.0, "AUD", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_bank_name.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_banks();
            }
        });
        ui.add_space(8.0);
        let banks: Vec<_> = self.bank_accounts.clone();
        egui::Grid::new("banks_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("name");
                ui.strong("currency");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for a in &banks {
                    let selected = self.selected_edit_bank_id.as_deref() == Some(a.id.as_str());
                    if ui.selectable_label(selected, &a.name).clicked() {
                        self.select_bank_for_edit(a);
                    }
                    ui.label(&a.currency);
                    ui.label(if a.active { "yes" } else { "no" });
                    ui.monospace(&a.id);
                    ui.end_row();
                }
            });
        if self.bank_accounts.is_empty() && self.banks_slot.is_none() {
            ui.label(egui::RichText::new("No bank accounts.").weak());
        }
        if self.selected_edit_bank_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit bank account").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Name");
                let id = egui::Id::new("edit_bank_name");
                focus_once(ui, id, &mut self.focus_edit_bank);
                text_field(ui, &mut self.edit_bank_name, 160.0, "Name", id, &mut focused);
                ui.label("Currency");
                let id = egui::Id::new("edit_bank_currency");
                text_field(ui, &mut self.edit_bank_currency, 60.0, "AUD", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_bank_active, "Active");
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_bank();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
    }

    pub(crate) fn draw_allocations_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Allocations");
        ui.add_space(4.0);
        self.draw_form_error(ui);
        let mut focused = false;
        ui.horizontal(|ui| {
            ui.label("Payment id");
            let id = egui::Id::new("alloc_payment_id");
            text_field(
                ui,
                &mut self.alloc_payment_id,
                200.0,
                "payment uuid",
                id,
                &mut focused,
            );
            ui.label("Document id");
            let id = egui::Id::new("alloc_document_id");
            text_field(
                ui,
                &mut self.alloc_document_id,
                200.0,
                "document uuid",
                id,
                &mut focused,
            );
            ui.label("Amount");
            let id = egui::Id::new("alloc_amount");
            text_field(ui, &mut self.alloc_amount, 80.0, "cents", id, &mut focused);
        });
        let keys = form_keys(ui, focused);
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                self.request_allocations();
            }
            if (keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Allocate").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !keys.cancel
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
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Code");
            let id = egui::Id::new("create_wh_code");
            focus_once(ui, id, &mut self.focus_create_wh);
            text_field(ui, &mut self.new_wh_code, 80.0, "Code", id, &mut create_focused);
            ui.label("Name");
            let id = egui::Id::new("create_wh_name");
            text_field(ui, &mut self.new_wh_name, 160.0, "Name", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_wh_code.clear();
                self.new_wh_name.clear();
            }
            if ui.button("Refresh").clicked() {
                self.inventory_error = None;
                self.request_warehouses();
            }
        });
        ui.add_space(8.0);
        let warehouses: Vec<_> = self.warehouses.clone();
        egui::Grid::new("wh_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("code");
                ui.strong("name");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for w in &warehouses {
                    let selected = self.selected_edit_warehouse_id.as_deref() == Some(w.id.as_str());
                    ui.monospace(&w.code);
                    if ui.selectable_label(selected, &w.name).clicked() {
                        self.select_warehouse_for_edit(w);
                    }
                    ui.label(if w.active { "yes" } else { "no" });
                    ui.monospace(&w.id);
                    ui.end_row();
                }
            });
        if self.warehouses.is_empty() && self.warehouses_slot.is_none() && self.inventory_error.is_none()
        {
            ui.label(egui::RichText::new("No warehouses.").weak());
        }
        if self.selected_edit_warehouse_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit warehouse").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Code");
                let id = egui::Id::new("edit_wh_code");
                focus_once(ui, id, &mut self.focus_edit_wh);
                text_field(ui, &mut self.edit_wh_code, 80.0, "Code", id, &mut focused);
                ui.label("Name");
                let id = egui::Id::new("edit_wh_name");
                text_field(ui, &mut self.edit_wh_name, 160.0, "Name", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_wh_active, "Active");
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_warehouse();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
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
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Product");
            let id = egui::Id::new("create_move_product");
            text_field(
                ui,
                &mut self.new_move_product_id,
                180.0,
                "product uuid",
                id,
                &mut create_focused,
            );
            ui.label("Qty");
            let id = egui::Id::new("create_move_qty");
            text_field(ui, &mut self.new_move_qty, 60.0, "qty", id, &mut create_focused);
            ui.label("To warehouse");
            let id = egui::Id::new("create_move_wh");
            text_field(
                ui,
                &mut self.new_move_wh_id,
                180.0,
                "warehouse uuid",
                id,
                &mut create_focused,
            );
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create move").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
        let module_focused = self.selected_module_id.is_some();
        let module_keys = form_keys(ui, module_focused);
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
                    let selected = self.selected_module_id.as_deref() == Some(row.id.as_str());
                    if ui.selectable_label(selected, &row.id).clicked() {
                        self.selected_module_id = Some(row.id.clone());
                    }
                    ui.label(&row.name);
                    ui.label(if row.enabled { "yes" } else { "no" });
                    ui.label(if row.always_on { "yes" } else { "no" });
                    if row.always_on {
                        ui.label("—");
                    } else {
                        let label = if row.enabled { "Disable" } else { "Enable" };
                        let activate = module_keys.submit
                            && selected
                            && live_grpc_supported()
                            && self.mutate_slot.is_none();
                        if activate
                            || ui
                                .add_enabled(
                                    live_grpc_supported() && self.mutate_slot.is_none(),
                                    egui::Button::new(label),
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
        let mut create_focused = false;
        ui.horizontal(|ui| {
            ui.label("Login");
            let id = egui::Id::new("create_user_login");
            focus_once(ui, id, &mut self.focus_create_user);
            text_field(ui, &mut self.new_user_login, 100.0, "Login", id, &mut create_focused);
            ui.label("Display");
            let id = egui::Id::new("create_user_display");
            text_field(ui, &mut self.new_user_display, 120.0, "Display", id, &mut create_focused);
            ui.label("Password");
            let id = egui::Id::new("create_user_password");
            text_field(ui, &mut self.new_user_password, 100.0, "Password", id, &mut create_focused);
        });
        let create_keys = form_keys(ui, create_focused);
        ui.horizontal(|ui| {
            if (create_keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create user").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !create_keys.cancel
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
            if create_keys.cancel {
                self.new_user_login.clear();
                self.new_user_display.clear();
                self.new_user_password.clear();
            }
            if ui.button("Refresh").clicked() {
                self.request_users_roles();
            }
        });
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Users").strong());
        let users: Vec<_> = self.users.clone();
        egui::Grid::new("users_grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("login");
                ui.strong("display");
                ui.strong("active");
                ui.strong("id");
                ui.end_row();
                for u in &users {
                    let selected = self.selected_edit_user_id.as_deref() == Some(u.id.as_str());
                    ui.monospace(&u.login);
                    if ui.selectable_label(selected, &u.display_name).clicked() {
                        self.select_user_for_edit(u);
                    }
                    ui.label(if u.active { "yes" } else { "no" });
                    ui.monospace(&u.id);
                    ui.end_row();
                }
            });
        if self.selected_edit_user_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit user").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Display");
                let id = egui::Id::new("edit_user_display");
                focus_once(ui, id, &mut self.focus_edit_user);
                text_field(ui, &mut self.edit_user_display, 160.0, "Display", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("New password");
                let id = egui::Id::new("edit_user_password");
                text_field(
                    ui,
                    &mut self.edit_user_password,
                    120.0,
                    "leave blank to keep",
                    id,
                    &mut focused,
                );
            });
            ui.checkbox(&mut self.edit_user_active, "Active");
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.cancel_edit();
            }
            ui.horizontal(|ui| {
                if (keys.submit
                    || ui
                        .add_enabled(
                            live_grpc_supported() && self.mutate_slot.is_none(),
                            egui::Button::new("Save").fill(tokens::ACCENT),
                        )
                        .clicked())
                    && !keys.cancel
                {
                    self.submit_edit_user();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
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

    fn select_product_for_edit(&mut self, p: &ProductRow) {
        self.selected_edit_product_id = Some(p.id.clone());
        self.edit_product_sku = p.sku.clone();
        self.edit_product_name = p.name.clone();
        self.edit_product_active = p.active;
        self.focus_edit_product = true;
    }

    fn submit_edit_product(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_product_id.clone() else {
            return;
        };
        let sku = self.edit_product_sku.trim().to_string();
        let name = self.edit_product_name.trim().to_string();
        if sku.is_empty() || name.is_empty() {
            self.form_error = Some("SKU and name required".into());
            return;
        }
        let active = self.edit_product_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_product(&mut conn, id, sku, name, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_category_for_edit(&mut self, c: &CategoryRow) {
        self.selected_edit_category_id = Some(c.id.clone());
        self.edit_category_name = c.name.clone();
        self.edit_category_active = c.active;
        self.focus_edit_category = true;
    }

    fn submit_edit_category(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_category_id.clone() else {
            return;
        };
        let name = self.edit_category_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Name required".into());
            return;
        }
        let active = self.edit_category_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_category(&mut conn, id, name, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_sales_for_edit(&mut self, d: &SalesDocRow) {
        self.selected_edit_sales_id = Some(d.id.clone());
        self.edit_sales_notes = d.notes.clone();
        self.edit_sales_status = d.status.clone();
        self.focus_edit_sales = true;
    }

    fn submit_edit_sales(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_sales_id.clone() else {
            return;
        };
        let notes = self.edit_sales_notes.clone();
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_sales_document(&mut conn, id, notes)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn submit_sales_status(&mut self, status: DocumentStatus) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_sales_id.clone() else {
            return;
        };
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = set_sales_document_status(&mut conn, id, status)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_payment_for_edit(&mut self, p: &PaymentRow) {
        self.selected_edit_payment_id = Some(p.id.clone());
        self.edit_payment_ref = p.reference.clone();
        self.focus_edit_payment = true;
    }

    fn submit_edit_payment(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_payment_id.clone() else {
            return;
        };
        let reference = self.edit_payment_ref.clone();
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_payment(&mut conn, id, reference)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_bank_for_edit(&mut self, a: &BankAccountRow) {
        self.selected_edit_bank_id = Some(a.id.clone());
        self.edit_bank_name = a.name.clone();
        self.edit_bank_currency = a.currency.clone();
        self.edit_bank_active = a.active;
        self.focus_edit_bank = true;
    }

    fn submit_edit_bank(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_bank_id.clone() else {
            return;
        };
        let name = self.edit_bank_name.trim().to_string();
        let currency = self.edit_bank_currency.trim().to_string();
        if name.is_empty() || currency.len() != 3 {
            self.form_error = Some("Name and 3-letter currency required".into());
            return;
        }
        let active = self.edit_bank_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_bank_account(&mut conn, id, name, currency, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_warehouse_for_edit(&mut self, w: &WarehouseRow) {
        self.selected_edit_warehouse_id = Some(w.id.clone());
        self.edit_wh_code = w.code.clone();
        self.edit_wh_name = w.name.clone();
        self.edit_wh_active = w.active;
        self.focus_edit_wh = true;
    }

    fn submit_edit_warehouse(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_warehouse_id.clone() else {
            return;
        };
        let code = self.edit_wh_code.trim().to_string();
        let name = self.edit_wh_name.trim().to_string();
        if code.is_empty() || name.is_empty() {
            self.form_error = Some("Code and name required".into());
            return;
        }
        let active = self.edit_wh_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_warehouse(&mut conn, id, code, name, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_user_for_edit(&mut self, u: &UserRow) {
        self.selected_edit_user_id = Some(u.id.clone());
        self.edit_user_display = u.display_name.clone();
        self.edit_user_active = u.active;
        self.edit_user_password.clear();
        self.focus_edit_user = true;
    }

    fn submit_edit_user(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_user_id.clone() else {
            return;
        };
        let display = self.edit_user_display.trim().to_string();
        if display.is_empty() {
            self.form_error = Some("Display name required".into());
            return;
        }
        let active = self.edit_user_active;
        let password = if self.edit_user_password.trim().is_empty() {
            None
        } else {
            Some(self.edit_user_password.clone())
        };
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_user(&mut conn, id, display, active, password)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }
}
