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
}

/// Top-level functional domain (icon rail).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    /// Placeholder — not enabled this phase.
    Home,
    Parties,
    /// Placeholder — not enabled this phase.
    Catalog,
    /// Placeholder — not enabled this phase.
    Sales,
    Settings,
}

impl Domain {
    pub fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Parties => "Parties",
            Self::Catalog => "Catalog",
            Self::Sales => "Sales",
            Self::Settings => "Settings",
        }
    }

    /// Unicode glyph for the rail (no web font / Material Symbols).
    pub fn icon(self) -> &'static str {
        match self {
            Self::Home => "⌂",
            Self::Parties => "♟",
            Self::Catalog => "☰",
            Self::Sales => "¤",
            Self::Settings => "⚙",
        }
    }

    pub fn is_enabled(self) -> bool {
        matches!(self, Self::Parties | Self::Settings)
    }

    /// Rail order: main domains top → bottom; Settings is pinned separately.
    pub fn main_rail() -> &'static [Domain] {
        &[Self::Home, Self::Parties, Self::Catalog, Self::Sales]
    }
}

/// Page within a domain (domain menu column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    AllParties,
    /// Same list as All Parties for now (no filter RPC).
    Customers,
    /// Same list as All Parties for now (no filter RPC).
    Suppliers,
    /// Settings host (tabs live on this page).
    SettingsHost,
}

impl Page {
    pub fn label(self) -> &'static str {
        match self {
            Self::AllParties => "All Parties",
            Self::Customers => "Customers",
            Self::Suppliers => "Suppliers",
            Self::SettingsHost => "Settings",
        }
    }

    /// Whether this page shows a multi-pane tab strip.
    pub fn shows_tabs(self) -> bool {
        matches!(self, Self::SettingsHost)
    }
}

/// Tab panes on multi-pane pages (Settings).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SettingsTab {
    #[default]
    Connection,
    About,
}

impl SettingsTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::Connection => "Connection",
            Self::About => "About",
        }
    }

    pub fn all() -> &'static [SettingsTab] {
        &[Self::Connection, Self::About]
    }
}

/// Menu rows for a domain.
pub fn pages_for_domain(domain: Domain) -> &'static [Page] {
    match domain {
        Domain::Parties => &[Page::AllParties, Page::Customers, Page::Suppliers],
        Domain::Settings => &[Page::SettingsHost],
        Domain::Home | Domain::Catalog | Domain::Sales => &[],
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
    /// Select an enabled domain and its default page/tab.
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

    /// Parties list pages (All / Customers / Suppliers) share the Phase 1 list view.
    pub fn shows_parties_list(&self) -> bool {
        matches!(
            self.selected_page,
            Page::AllParties | Page::Customers | Page::Suppliers
        )
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
    fn placeholder_domains_rejected() {
        let mut nav = ShellNav::default();
        assert!(!nav.select_domain(Domain::Home));
        assert!(!nav.select_domain(Domain::Catalog));
        assert!(!nav.select_domain(Domain::Sales));
        assert_eq!(nav.selected_domain, Domain::Parties);
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
        nav.select_tab(SettingsTab::About);
        assert_eq!(nav.selected_tab, SettingsTab::About);
        nav.select_tab(SettingsTab::Connection);
        assert_eq!(nav.selected_tab, SettingsTab::Connection);
    }

    #[test]
    fn single_pane_parties_has_no_tabs() {
        assert!(!Page::AllParties.shows_tabs());
        assert!(!Page::Customers.shows_tabs());
        assert!(Page::SettingsHost.shows_tabs());
    }
}
