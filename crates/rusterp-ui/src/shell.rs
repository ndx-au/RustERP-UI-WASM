//! App shell navigation model (domains / pages / tabs). Presentation only.

/// Fixed chrome sizes from Spec / `stitch/` token guidance (approximate).
pub mod tokens {
    pub const RAIL_WIDTH: f32 = 56.0;
    pub const MENU_WIDTH: f32 = 200.0;
    pub const TOP_BAR_HEIGHT: f32 = 48.0;
    pub const PANE_PADDING: f32 = 16.0;
    pub const DENSE_ROW: f32 = 28.0;
    /// Active rail indicator + primary actions (rust-orange intent).
    pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(0xce, 0x41, 0x2b);
    pub const SURFACE: egui::Color32 = egui::Color32::from_rgb(0x13, 0x13, 0x13);
    pub const SURFACE_RAIL: egui::Color32 = egui::Color32::from_rgb(0x0e, 0x0e, 0x0e);
    pub const SURFACE_MENU: egui::Color32 = egui::Color32::from_rgb(0x1c, 0x1b, 0x1b);
    pub const SURFACE_TOP: egui::Color32 = egui::Color32::from_rgb(0x20, 0x1f, 0x1f);
    pub const ERROR: egui::Color32 = egui::Color32::from_rgb(0xff, 0xb4, 0xab);
    pub const WIREFRAME_MUTED: egui::Color32 = egui::Color32::from_rgb(0x70, 0x70, 0x70);
}

/// Navigation tier — maps to PostgreSQL schema maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainTier {
    /// Live gRPC-backed content (Parties list, Settings connection).
    Live,
    /// MVP schema present; wireframe stub content.
    Wireframe,
    /// Post-MVP stub tables only.
    FutureStub,
}

/// Metadata for wireframe content panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireframeMeta {
    pub schema_path: &'static str,
    pub tier_label: &'static str,
    pub description: &'static str,
}

/// Top-level functional domain (icon rail).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    Home,
    Parties,
    Catalog,
    Sales,
    Payments,
    Inventory,
    Purchasing,
    Accounting,
    Crm,
    Projects,
    Hr,
    Manufacturing,
    Settings,
}

impl Domain {
    pub fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Parties => "Parties",
            Self::Catalog => "Catalog",
            Self::Sales => "Sales",
            Self::Payments => "Payments",
            Self::Inventory => "Inventory",
            Self::Purchasing => "Purchasing",
            Self::Accounting => "Accounting",
            Self::Crm => "CRM",
            Self::Projects => "Projects",
            Self::Hr => "HR",
            Self::Manufacturing => "Manufacturing",
            Self::Settings => "Settings",
        }
    }

    /// Unicode rail icon glyphs with embedded fallback font support.
    pub fn icon(self) -> &'static str {
        match self {
            Self::Home => "⌂",
            Self::Parties => "♟",
            Self::Catalog => "☰",
            Self::Sales => "¤",
            Self::Payments => "⊕",
            Self::Inventory => "⌗",
            Self::Purchasing => "⇩",
            Self::Accounting => "Σ",
            Self::Crm => "☎",
            Self::Projects => "▣",
            Self::Hr => "⚲",
            Self::Manufacturing => "⚒",
            Self::Settings => "⚙",
        }
    }

    pub fn tier(self) -> DomainTier {
        match self {
            Self::Parties | Self::Settings => DomainTier::Live,
            Self::Home
            | Self::Catalog
            | Self::Sales
            | Self::Payments
            | Self::Inventory => DomainTier::Wireframe,
            Self::Purchasing
            | Self::Accounting
            | Self::Crm
            | Self::Projects
            | Self::Hr
            | Self::Manufacturing => DomainTier::FutureStub,
        }
    }

    /// Wireframe pass: all domains are navigable.
    pub fn is_enabled(self) -> bool {
        true
    }

    pub fn rail_tooltip(self) -> String {
        let label = self.label();
        match self.tier() {
            DomainTier::Live => label.to_string(),
            DomainTier::Wireframe => format!("{label} — wireframe"),
            DomainTier::FutureStub => format!("{label} — Post-MVP"),
        }
    }

    pub fn mvp_rail() -> &'static [Domain] {
        &[
            Self::Home,
            Self::Parties,
            Self::Catalog,
            Self::Sales,
            Self::Payments,
            Self::Inventory,
        ]
    }

    pub fn future_rail() -> &'static [Domain] {
        &[
            Self::Purchasing,
            Self::Accounting,
            Self::Crm,
            Self::Projects,
            Self::Hr,
            Self::Manufacturing,
        ]
    }
}

/// Page within a domain (domain menu column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    // Home
    Dashboard,
    // Parties
    AllParties,
    Customers,
    Suppliers,
    Prospects,
    Contacts,
    Addresses,
    // Catalog
    Products,
    Categories,
    UnitsOfMeasure,
    PriceLists,
    // Sales
    Quotes,
    Orders,
    Invoices,
    CreditNotes,
    // Payments
    PaymentsList,
    BankAccounts,
    Allocations,
    // Inventory
    Warehouses,
    StockLevels,
    StockMoves,
    // Purchasing
    PurchaseOrders,
    // Accounting
    ChartOfAccounts,
    // CRM
    Activities,
    // Projects
    ProjectsList,
    // HR
    Employees,
    // Manufacturing
    BillsOfMaterials,
    // Settings
    SettingsHost,
}

impl Page {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::AllParties => "All Parties",
            Self::Customers => "Customers",
            Self::Suppliers => "Suppliers",
            Self::Prospects => "Prospects",
            Self::Contacts => "Contacts",
            Self::Addresses => "Addresses",
            Self::Products => "Products",
            Self::Categories => "Categories",
            Self::UnitsOfMeasure => "Units of Measure",
            Self::PriceLists => "Price Lists",
            Self::Quotes => "Quotes",
            Self::Orders => "Orders",
            Self::Invoices => "Invoices",
            Self::CreditNotes => "Credit Notes",
            Self::PaymentsList => "Payments",
            Self::BankAccounts => "Bank Accounts",
            Self::Allocations => "Allocations",
            Self::Warehouses => "Warehouses",
            Self::StockLevels => "Stock Levels",
            Self::StockMoves => "Stock Moves",
            Self::PurchaseOrders => "Purchase Orders",
            Self::ChartOfAccounts => "Chart of Accounts",
            Self::Activities => "Activities",
            Self::ProjectsList => "Projects",
            Self::Employees => "Employees",
            Self::BillsOfMaterials => "Bills of Materials",
            Self::SettingsHost => "Settings",
        }
    }

    pub fn shows_tabs(self) -> bool {
        matches!(self, Self::SettingsHost)
    }

    pub fn is_live_parties_list(self) -> bool {
        matches!(self, Self::AllParties | Self::Customers | Self::Suppliers)
    }

    pub fn wireframe_meta(self) -> Option<WireframeMeta> {
        if self.is_live_parties_list() || self == Self::SettingsHost {
            return None;
        }
        Some(match self {
            Self::Dashboard => WireframeMeta {
                schema_path: "core.settings",
                tier_label: "MVP schema",
                description: "Org overview and key metrics — platform dashboard.",
            },
            Self::Prospects => WireframeMeta {
                schema_path: "party.parties + party.party_roles",
                tier_label: "MVP schema",
                description: "Parties with prospect role filter.",
            },
            Self::Contacts => WireframeMeta {
                schema_path: "party.contacts",
                tier_label: "MVP schema",
                description: "Contacts belonging to parties.",
            },
            Self::Addresses => WireframeMeta {
                schema_path: "party.addresses",
                tier_label: "MVP schema",
                description: "Billing, shipping, and other addresses.",
            },
            Self::Products => WireframeMeta {
                schema_path: "catalog.products",
                tier_label: "MVP schema",
                description: "Stock, service, and consumable products.",
            },
            Self::Categories => WireframeMeta {
                schema_path: "catalog.product_categories",
                tier_label: "MVP schema",
                description: "Hierarchical product categories.",
            },
            Self::UnitsOfMeasure => WireframeMeta {
                schema_path: "catalog.units_of_measure",
                tier_label: "MVP schema",
                description: "Units of measure for products and lines.",
            },
            Self::PriceLists => WireframeMeta {
                schema_path: "catalog.price_lists / catalog.prices",
                tier_label: "MVP schema",
                description: "Named price lists and product prices.",
            },
            Self::Quotes => WireframeMeta {
                schema_path: "sales.sales_documents (kind = quote)",
                tier_label: "MVP schema",
                description: "Sales quotes — first step in the pipeline.",
            },
            Self::Orders => WireframeMeta {
                schema_path: "sales.sales_documents (kind = order)",
                tier_label: "MVP schema",
                description: "Confirmed sales orders.",
            },
            Self::Invoices => WireframeMeta {
                schema_path: "sales.sales_documents (kind = invoice)",
                tier_label: "MVP schema",
                description: "Posted customer invoices.",
            },
            Self::CreditNotes => WireframeMeta {
                schema_path: "sales.sales_documents (kind = credit_note)",
                tier_label: "MVP schema",
                description: "Credit notes against invoices.",
            },
            Self::PaymentsList => WireframeMeta {
                schema_path: "payment.payments",
                tier_label: "MVP schema",
                description: "Inbound and outbound payment records.",
            },
            Self::BankAccounts => WireframeMeta {
                schema_path: "payment.bank_accounts",
                tier_label: "MVP schema",
                description: "Organisation bank accounts.",
            },
            Self::Allocations => WireframeMeta {
                schema_path: "payment.payment_allocations",
                tier_label: "MVP schema",
                description: "Payment allocations to invoices.",
            },
            Self::Warehouses => WireframeMeta {
                schema_path: "inventory.warehouses",
                tier_label: "MVP schema",
                description: "Warehouse and location master data.",
            },
            Self::StockLevels => WireframeMeta {
                schema_path: "inventory.stock_levels",
                tier_label: "MVP schema",
                description: "On-hand and reserved quantities per warehouse.",
            },
            Self::StockMoves => WireframeMeta {
                schema_path: "inventory.stock_moves",
                tier_label: "MVP schema",
                description: "Stock transfers and delivery moves.",
            },
            Self::PurchaseOrders => WireframeMeta {
                schema_path: "purchase.purchase_orders",
                tier_label: "Post-MVP stub",
                description: "Purchase order header (stub table in migration 0009).",
            },
            Self::ChartOfAccounts => WireframeMeta {
                schema_path: "accounting.accounts",
                tier_label: "Post-MVP stub",
                description: "Chart of accounts stub — not full double-entry GL.",
            },
            Self::Activities => WireframeMeta {
                schema_path: "crm.activities",
                tier_label: "Post-MVP stub",
                description: "CRM activities and follow-ups.",
            },
            Self::ProjectsList => WireframeMeta {
                schema_path: "project.projects",
                tier_label: "Post-MVP stub",
                description: "Project management stub.",
            },
            Self::Employees => WireframeMeta {
                schema_path: "hr.employees",
                tier_label: "Post-MVP stub",
                description: "HR employee records stub.",
            },
            Self::BillsOfMaterials => WireframeMeta {
                schema_path: "manufacturing.boms",
                tier_label: "Post-MVP stub",
                description: "Bills of materials stub.",
            },
            Self::AllParties | Self::Customers | Self::Suppliers | Self::SettingsHost => {
                return None;
            }
        })
    }
}

/// Tab panes on multi-pane pages (Settings).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SettingsTab {
    #[default]
    Connection,
    Modules,
    UsersAndRoles,
    About,
}

impl SettingsTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::Connection => "Connection",
            Self::Modules => "Modules",
            Self::UsersAndRoles => "Users & Roles",
            Self::About => "About",
        }
    }

    pub fn all() -> &'static [SettingsTab] {
        &[
            Self::Connection,
            Self::Modules,
            Self::UsersAndRoles,
            Self::About,
        ]
    }
}

/// Menu rows for a domain.
pub fn pages_for_domain(domain: Domain) -> &'static [Page] {
    match domain {
        Domain::Home => &[Page::Dashboard],
        Domain::Parties => &[
            Page::AllParties,
            Page::Customers,
            Page::Suppliers,
            Page::Prospects,
            Page::Contacts,
            Page::Addresses,
        ],
        Domain::Catalog => &[
            Page::Products,
            Page::Categories,
            Page::UnitsOfMeasure,
            Page::PriceLists,
        ],
        Domain::Sales => &[
            Page::Quotes,
            Page::Orders,
            Page::Invoices,
            Page::CreditNotes,
        ],
        Domain::Payments => &[
            Page::PaymentsList,
            Page::BankAccounts,
            Page::Allocations,
        ],
        Domain::Inventory => &[
            Page::Warehouses,
            Page::StockLevels,
            Page::StockMoves,
        ],
        Domain::Purchasing => &[Page::PurchaseOrders],
        Domain::Accounting => &[Page::ChartOfAccounts],
        Domain::Crm => &[Page::Activities],
        Domain::Projects => &[Page::ProjectsList],
        Domain::Hr => &[Page::Employees],
        Domain::Manufacturing => &[Page::BillsOfMaterials],
        Domain::Settings => &[Page::SettingsHost],
    }
}

/// Default page when entering a domain.
pub fn default_page(domain: Domain) -> Option<Page> {
    pages_for_domain(domain).first().copied()
}

/// Shell navigation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellNav {
    pub selected_domain: Domain,
    pub selected_page: Page,
    pub selected_tab: SettingsTab,
}

impl Default for ShellNav {
    fn default() -> Self {
        Self {
            selected_domain: Domain::Parties,
            selected_page: Page::AllParties,
            selected_tab: SettingsTab::Connection,
        }
    }
}

impl ShellNav {
    /// Select a domain and its default page/tab.
    pub fn select_domain(&mut self, domain: Domain) -> bool {
        if !domain.is_enabled() {
            return false;
        }
        self.selected_domain = domain;
        if let Some(page) = default_page(domain) {
            self.selected_page = page;
        }
        if domain == Domain::Settings {
            self.selected_tab = SettingsTab::Connection;
        }
        true
    }

    /// Select a page that belongs to the current domain.
    pub fn select_page(&mut self, page: Page) -> bool {
        if !pages_for_domain(self.selected_domain).contains(&page) {
            return false;
        }
        self.selected_page = page;
        if page == Page::SettingsHost {
            self.selected_tab = SettingsTab::Connection;
        }
        true
    }

    pub fn select_tab(&mut self, tab: SettingsTab) {
        self.selected_tab = tab;
    }

    /// Bottom sprocket: Settings domain + Connection tab.
    pub fn open_settings(&mut self) {
        let _ = self.select_domain(Domain::Settings);
    }

    pub fn shows_parties_list(&self) -> bool {
        self.selected_page.is_live_parties_list()
    }

    pub fn customers_suppliers_unfiltered_note(&self) -> Option<&'static str> {
        match self.selected_page {
            Page::Customers | Page::Suppliers => {
                Some("Role filter not enabled yet — showing full list from core.")
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_nav_is_parties_list() {
        let nav = ShellNav::default();
        assert_eq!(nav.selected_domain, Domain::Parties);
        assert_eq!(nav.selected_page, Page::AllParties);
        assert!(nav.shows_parties_list());
        assert!(!nav.selected_page.shows_tabs());
    }

    #[test]
    fn all_domains_navigable() {
        let mut nav = ShellNav::default();
        for domain in Domain::mvp_rail()
            .iter()
            .chain(Domain::future_rail().iter())
            .copied()
        {
            assert!(nav.select_domain(domain));
            assert_eq!(nav.selected_domain, domain);
            assert!(default_page(domain).is_some());
        }
    }

    #[test]
    fn settings_sprocket_opens_connection_tab() {
        let mut nav = ShellNav::default();
        nav.open_settings();
        assert_eq!(nav.selected_domain, Domain::Settings);
        assert_eq!(nav.selected_page, Page::SettingsHost);
        assert_eq!(nav.selected_tab, SettingsTab::Connection);
        assert!(nav.selected_page.shows_tabs());
    }

    #[test]
    fn page_must_belong_to_domain() {
        let mut nav = ShellNav::default();
        assert!(!nav.select_page(Page::SettingsHost));
        assert!(nav.select_domain(Domain::Settings));
        assert!(nav.select_page(Page::SettingsHost));
        assert!(!nav.select_page(Page::AllParties));
    }

    #[test]
    fn settings_tabs_switch() {
        let mut nav = ShellNav::default();
        nav.open_settings();
        nav.select_tab(SettingsTab::Modules);
        assert_eq!(nav.selected_tab, SettingsTab::Modules);
        nav.select_tab(SettingsTab::UsersAndRoles);
        assert_eq!(nav.selected_tab, SettingsTab::UsersAndRoles);
        nav.select_tab(SettingsTab::About);
        assert_eq!(nav.selected_tab, SettingsTab::About);
    }

    #[test]
    fn sales_pages_map_to_schema() {
        let pages = pages_for_domain(Domain::Sales);
        assert_eq!(pages.len(), 4);
        assert!(Page::Quotes.wireframe_meta().is_some());
        assert!(Page::Invoices.wireframe_meta().unwrap().schema_path.contains("invoice"));
    }

    #[test]
    fn live_parties_pages_have_no_wireframe_meta() {
        assert!(Page::AllParties.wireframe_meta().is_none());
        assert!(Page::Customers.wireframe_meta().is_none());
        assert!(Page::Prospects.wireframe_meta().is_some());
    }

    #[test]
    fn future_stub_tier() {
        assert_eq!(Domain::Manufacturing.tier(), DomainTier::FutureStub);
        assert_eq!(Domain::Catalog.tier(), DomainTier::Wireframe);
        assert_eq!(Domain::Parties.tier(), DomainTier::Live);
    }
}
