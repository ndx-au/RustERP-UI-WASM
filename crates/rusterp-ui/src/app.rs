//! Reference shell chrome: rail + domain menu + top bar + content host.

use crate::forms::{focus_once, form_keys, text_field};
use crate::shell::{
    pages_for_domain, tokens, Domain, DomainTier, Page, SettingsTab, ShellNav,
};
use crate::wireframe::draw_wireframe_stub;
use rusterp_api_client::{
    add_address, add_contact, create_party, default_rpc_url, list_addresses, list_contacts,
    live_grpc_supported, live_grpc_unavailable_reason, normalize_rpc_url, shared_result,
    spawn_local_fut, update_address, update_contact, update_party, AddressRow, AllocationRow,
    BankAccountRow, CategoryRow, Connection, ConnectionStatus, ContactRow, ModuleRow, PartyRole,
    PartyRow, PaymentRow, PermissionRow, ProductRow, RefreshSnapshot, RoleRow, SalesDocRow,
    SharedResult, StockLevelRow, StockMoveRow, UserRow, WarehouseRow, DEFAULT_RPC_URL,
    ENDPOINT_ENV, RPC_URL_ENV,
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
    // Edit — parties
    pub(crate) selected_edit_party_id: Option<String>,
    pub(crate) edit_party_name: String,
    pub(crate) edit_party_customer: bool,
    pub(crate) edit_party_supplier: bool,
    pub(crate) edit_party_prospect: bool,
    pub(crate) edit_party_active: bool,
    pub(crate) focus_create_party: bool,
    pub(crate) focus_edit_party: bool,
    // Edit — contacts
    pub(crate) selected_edit_contact_id: Option<String>,
    pub(crate) edit_contact_name: String,
    pub(crate) edit_contact_email: String,
    pub(crate) edit_contact_phone: String,
    pub(crate) edit_contact_active: bool,
    pub(crate) focus_create_contact: bool,
    pub(crate) focus_edit_contact: bool,
    // Edit — addresses
    pub(crate) selected_edit_address_id: Option<String>,
    pub(crate) edit_address_line1: String,
    pub(crate) edit_address_city: String,
    pub(crate) edit_address_country: String,
    pub(crate) edit_address_active: bool,
    pub(crate) focus_create_address: bool,
    pub(crate) focus_edit_address: bool,
    // Edit — catalog
    pub(crate) selected_edit_product_id: Option<String>,
    pub(crate) edit_product_sku: String,
    pub(crate) edit_product_name: String,
    pub(crate) edit_product_active: bool,
    pub(crate) focus_create_product: bool,
    pub(crate) focus_edit_product: bool,
    pub(crate) selected_edit_category_id: Option<String>,
    pub(crate) edit_category_name: String,
    pub(crate) edit_category_active: bool,
    pub(crate) focus_create_category: bool,
    pub(crate) focus_edit_category: bool,
    // Edit — sales
    pub(crate) selected_edit_sales_id: Option<String>,
    pub(crate) edit_sales_notes: String,
    pub(crate) edit_sales_status: String,
    pub(crate) focus_create_sales: bool,
    pub(crate) focus_edit_sales: bool,
    // Edit — payments
    pub(crate) selected_edit_payment_id: Option<String>,
    pub(crate) edit_payment_ref: String,
    pub(crate) focus_create_payment: bool,
    pub(crate) focus_edit_payment: bool,
    pub(crate) selected_edit_bank_id: Option<String>,
    pub(crate) edit_bank_name: String,
    pub(crate) edit_bank_currency: String,
    pub(crate) edit_bank_active: bool,
    pub(crate) focus_create_bank: bool,
    pub(crate) focus_edit_bank: bool,
    // Edit — inventory
    pub(crate) selected_edit_warehouse_id: Option<String>,
    pub(crate) edit_wh_code: String,
    pub(crate) edit_wh_name: String,
    pub(crate) edit_wh_active: bool,
    pub(crate) focus_create_wh: bool,
    pub(crate) focus_edit_wh: bool,
    // Edit — users
    pub(crate) selected_edit_user_id: Option<String>,
    pub(crate) edit_user_display: String,
    pub(crate) edit_user_active: bool,
    pub(crate) edit_user_password: String,
    pub(crate) focus_create_user: bool,
    pub(crate) focus_edit_user: bool,
    // Modules keyboard focus
    pub(crate) selected_module_id: Option<String>,
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
            selected_edit_party_id: None,
            edit_party_name: String::new(),
            edit_party_customer: false,
            edit_party_supplier: false,
            edit_party_prospect: false,
            edit_party_active: true,
            focus_create_party: false,
            focus_edit_party: false,
            selected_edit_contact_id: None,
            edit_contact_name: String::new(),
            edit_contact_email: String::new(),
            edit_contact_phone: String::new(),
            edit_contact_active: true,
            focus_create_contact: false,
            focus_edit_contact: false,
            selected_edit_address_id: None,
            edit_address_line1: String::new(),
            edit_address_city: String::new(),
            edit_address_country: String::new(),
            edit_address_active: true,
            focus_create_address: false,
            focus_edit_address: false,
            selected_edit_product_id: None,
            edit_product_sku: String::new(),
            edit_product_name: String::new(),
            edit_product_active: true,
            focus_create_product: false,
            focus_edit_product: false,
            selected_edit_category_id: None,
            edit_category_name: String::new(),
            edit_category_active: true,
            focus_create_category: false,
            focus_edit_category: false,
            selected_edit_sales_id: None,
            edit_sales_notes: String::new(),
            edit_sales_status: String::new(),
            focus_create_sales: false,
            focus_edit_sales: false,
            selected_edit_payment_id: None,
            edit_payment_ref: String::new(),
            focus_create_payment: false,
            focus_edit_payment: false,
            selected_edit_bank_id: None,
            edit_bank_name: String::new(),
            edit_bank_currency: String::new(),
            edit_bank_active: true,
            focus_create_bank: false,
            focus_edit_bank: false,
            selected_edit_warehouse_id: None,
            edit_wh_code: String::new(),
            edit_wh_name: String::new(),
            edit_wh_active: true,
            focus_create_wh: false,
            focus_edit_wh: false,
            selected_edit_user_id: None,
            edit_user_display: String::new(),
            edit_user_active: true,
            edit_user_password: String::new(),
            focus_create_user: false,
            focus_edit_user: false,
            selected_module_id: None,
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

    pub(crate) fn clear_edit_selections(&mut self) {
        self.selected_edit_party_id = None;
        self.selected_edit_contact_id = None;
        self.selected_edit_address_id = None;
        self.selected_edit_product_id = None;
        self.selected_edit_category_id = None;
        self.selected_edit_sales_id = None;
        self.selected_edit_payment_id = None;
        self.selected_edit_bank_id = None;
        self.selected_edit_warehouse_id = None;
        self.selected_edit_user_id = None;
        self.selected_module_id = None;
    }

    pub(crate) fn cancel_edit(&mut self) {
        self.clear_edit_selections();
        self.form_error = None;
    }

    fn party_roles_from_flags(&self, customer: bool, supplier: bool, prospect: bool) -> Vec<PartyRole> {
        let mut roles = Vec::new();
        if customer {
            roles.push(PartyRole::Customer);
        }
        if supplier {
            roles.push(PartyRole::Supplier);
        }
        if prospect {
            roles.push(PartyRole::Prospect);
        }
        roles
    }

    fn select_party_for_edit(&mut self, p: &PartyRow) {
        self.selected_edit_party_id = Some(p.id.clone());
        self.edit_party_name = p.display_name.clone();
        self.edit_party_customer = p.roles.iter().any(|r| r == "customer");
        self.edit_party_supplier = p.roles.iter().any(|r| r == "supplier");
        self.edit_party_prospect = p.roles.iter().any(|r| r == "prospect");
        self.edit_party_active = p.active;
        self.focus_edit_party = true;
    }

    fn submit_edit_party(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_party_id.clone() else {
            return;
        };
        let name = self.edit_party_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Display name is required".into());
            return;
        }
        let roles = self.party_roles_from_flags(
            self.edit_party_customer,
            self.edit_party_supplier,
            self.edit_party_prospect,
        );
        if roles.is_empty() {
            self.form_error = Some("Select at least one role".into());
            return;
        }
        let active = self.edit_party_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_party(&mut conn, id, name, roles, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_contact_for_edit(&mut self, c: &ContactRow) {
        self.selected_edit_contact_id = Some(c.id.clone());
        self.edit_contact_name = c.name.clone();
        self.edit_contact_email = c.email.clone();
        self.edit_contact_phone = c.phone.clone();
        self.edit_contact_active = c.active;
        self.focus_edit_contact = true;
    }

    fn submit_edit_contact(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_contact_id.clone() else {
            return;
        };
        let name = self.edit_contact_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Contact name is required".into());
            return;
        }
        let email = self.edit_contact_email.clone();
        let phone = self.edit_contact_phone.clone();
        let active = self.edit_contact_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_contact(&mut conn, id, name, email, phone, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
    }

    fn select_address_for_edit(&mut self, a: &AddressRow) {
        self.selected_edit_address_id = Some(a.id.clone());
        self.edit_address_line1 = a.line1.clone();
        self.edit_address_city = a.city.clone();
        self.edit_address_country = a.country.clone();
        self.edit_address_active = a.active;
        self.focus_edit_address = true;
    }

    fn submit_edit_address(&mut self) {
        if self.mutate_slot.is_some() || !live_grpc_supported() {
            return;
        }
        let Some(id) = self.selected_edit_address_id.clone() else {
            return;
        };
        let line1 = self.edit_address_line1.trim().to_string();
        let city = self.edit_address_city.trim().to_string();
        if line1.is_empty() || city.is_empty() {
            self.form_error = Some("Line1 and city are required".into());
            return;
        }
        let country = self.edit_address_country.clone();
        let active = self.edit_address_active;
        let slot = shared_result();
        self.mutate_slot = Some(slot.clone());
        let url = self.rpc_url.clone();
        spawn_local_fut(async move {
            let mut conn = Connection::new(url);
            let result = update_address(&mut conn, id, line1, city, country, active)
                .await
                .map(|_| ());
            if let Ok(mut g) = slot.lock() {
                *g = Some(Ok(result));
            }
        });
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
                self.clear_edit_selections();
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
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Name");
                let id = egui::Id::new("create_party_name");
                focus_once(ui, id, &mut self.focus_create_party);
                text_field(ui, &mut self.new_party_name, 200.0, "Name", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.new_party_customer, "Customer");
                ui.checkbox(&mut self.new_party_supplier, "Supplier");
                ui.checkbox(&mut self.new_party_prospect, "Prospect");
            });
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.new_party_name.clear();
            }
            if (keys.submit
                || ui
                    .add_enabled(
                        live_grpc_supported() && self.mutate_slot.is_none(),
                        egui::Button::new("Create").fill(tokens::ACCENT),
                    )
                    .clicked())
                && !keys.cancel
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
                let parties: Vec<_> = self.parties.clone();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("parties_grid")
                        .striped(true)
                        .num_columns(4)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            ui.strong("Display name");
                            ui.strong("Roles");
                            ui.strong("Active");
                            ui.strong("Id");
                            ui.end_row();
                            for p in &parties {
                                let selected = self.selected_edit_party_id.as_deref() == Some(p.id.as_str());
                                if ui.selectable_label(selected, &p.display_name).clicked() {
                                    self.select_party_for_edit(p);
                                }
                                ui.label(p.roles.join(", "));
                                ui.label(if p.active { "yes" } else { "no" });
                                ui.monospace(&p.id);
                                ui.end_row();
                            }
                        });
                });

                if self.selected_edit_party_id.is_some() {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(egui::RichText::new("Edit party").strong());
                    let mut focused = false;
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        let id = egui::Id::new("edit_party_name");
                        focus_once(ui, id, &mut self.focus_edit_party);
                        text_field(ui, &mut self.edit_party_name, 200.0, "Name", id, &mut focused);
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.edit_party_customer, "Customer");
                        ui.checkbox(&mut self.edit_party_supplier, "Supplier");
                        ui.checkbox(&mut self.edit_party_prospect, "Prospect");
                        ui.checkbox(&mut self.edit_party_active, "Active");
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
                            self.submit_edit_party();
                        }
                        if ui.button("Cancel").clicked() {
                            self.cancel_edit();
                        }
                    });
                }
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
                        self.cancel_edit();
                        self.request_contacts(p.id);
                    }
                }
            });

        ui.add_space(8.0);
        ui.collapsing("Add contact", |ui| {
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Name");
                let id = egui::Id::new("create_contact_name");
                focus_once(ui, id, &mut self.focus_create_contact);
                text_field(ui, &mut self.new_contact_name, 160.0, "Name", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Email");
                let id = egui::Id::new("create_contact_email");
                text_field(ui, &mut self.new_contact_email, 160.0, "Email", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Phone");
                let id = egui::Id::new("create_contact_phone");
                text_field(ui, &mut self.new_contact_phone, 120.0, "Phone", id, &mut focused);
            });
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.new_contact_name.clear();
                self.new_contact_email.clear();
                self.new_contact_phone.clear();
            }
            if (keys.submit || ui.button("Add").clicked()) && !keys.cancel {
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
        let contacts: Vec<_> = self.contacts.clone();
        egui::Grid::new("contacts_grid")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("Name");
                ui.strong("Email");
                ui.strong("Phone");
                ui.strong("Active");
                ui.strong("Id");
                ui.end_row();
                for c in &contacts {
                    let selected = self.selected_edit_contact_id.as_deref() == Some(c.id.as_str());
                    if ui.selectable_label(selected, &c.name).clicked() {
                        self.select_contact_for_edit(c);
                    }
                    ui.label(&c.email);
                    ui.label(&c.phone);
                    ui.label(if c.active { "yes" } else { "no" });
                    ui.monospace(&c.id);
                    ui.end_row();
                }
            });

        if self.selected_edit_contact_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit contact").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Name");
                let id = egui::Id::new("edit_contact_name");
                focus_once(ui, id, &mut self.focus_edit_contact);
                text_field(ui, &mut self.edit_contact_name, 160.0, "Name", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Email");
                let id = egui::Id::new("edit_contact_email");
                text_field(ui, &mut self.edit_contact_email, 160.0, "Email", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Phone");
                let id = egui::Id::new("edit_contact_phone");
                text_field(ui, &mut self.edit_contact_phone, 120.0, "Phone", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_contact_active, "Active");
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
                    self.submit_edit_contact();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
        }
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
                        self.cancel_edit();
                        self.request_addresses(p.id);
                    }
                }
            });

        ui.add_space(8.0);
        ui.collapsing("Add address (billing)", |ui| {
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Line 1");
                let id = egui::Id::new("create_address_line1");
                focus_once(ui, id, &mut self.focus_create_address);
                text_field(ui, &mut self.new_address_line1, 180.0, "Line 1", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("City");
                let id = egui::Id::new("create_address_city");
                text_field(ui, &mut self.new_address_city, 120.0, "City", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Country");
                let id = egui::Id::new("create_address_country");
                text_field(ui, &mut self.new_address_country, 60.0, "AU", id, &mut focused);
            });
            let keys = form_keys(ui, focused);
            if keys.cancel {
                self.new_address_line1.clear();
                self.new_address_city.clear();
            }
            if (keys.submit || ui.button("Add").clicked()) && !keys.cancel {
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
        let addresses: Vec<_> = self.addresses.clone();
        egui::Grid::new("addresses_grid")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.strong("Kind");
                ui.strong("Line 1");
                ui.strong("City");
                ui.strong("Country");
                ui.strong("Active");
                ui.strong("Id");
                ui.end_row();
                for a in &addresses {
                    let selected = self.selected_edit_address_id.as_deref() == Some(a.id.as_str());
                    ui.label(&a.kind);
                    if ui.selectable_label(selected, &a.line1).clicked() {
                        self.select_address_for_edit(a);
                    }
                    ui.label(&a.city);
                    ui.label(&a.country);
                    ui.label(if a.active { "yes" } else { "no" });
                    ui.monospace(&a.id);
                    ui.end_row();
                }
            });

        if self.selected_edit_address_id.is_some() {
            ui.add_space(8.0);
            ui.separator();
            ui.label(egui::RichText::new("Edit address").strong());
            let mut focused = false;
            ui.horizontal(|ui| {
                ui.label("Line 1");
                let id = egui::Id::new("edit_address_line1");
                focus_once(ui, id, &mut self.focus_edit_address);
                text_field(ui, &mut self.edit_address_line1, 180.0, "Line 1", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("City");
                let id = egui::Id::new("edit_address_city");
                text_field(ui, &mut self.edit_address_city, 120.0, "City", id, &mut focused);
            });
            ui.horizontal(|ui| {
                ui.label("Country");
                let id = egui::Id::new("edit_address_country");
                text_field(ui, &mut self.edit_address_country, 60.0, "AU", id, &mut focused);
            });
            ui.checkbox(&mut self.edit_address_active, "Active");
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
                    self.submit_edit_address();
                }
                if ui.button("Cancel").clicked() {
                    self.cancel_edit();
                }
            });
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
