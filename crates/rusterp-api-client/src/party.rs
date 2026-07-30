//! Thin party view-models for the reference list UI (no domain logic).

/// One row in the Parties list (display mapping only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyRow {
    pub id: String,
    pub display_name: String,
    /// Human-readable role labels (e.g. `customer`, `supplier`).
    pub roles: Vec<String>,
    pub active: bool,
}

/// Map a `rusterp.party.v1.PartyRole` wire value to a short label.
///
/// Values match the vendored proto enum (not invented domain rules).
pub fn party_role_label(role: i32) -> &'static str {
    match role {
        1 => "customer",  // PARTY_ROLE_CUSTOMER
        2 => "supplier",  // PARTY_ROLE_SUPPLIER
        3 => "prospect",  // PARTY_ROLE_PROSPECT
        _ => "unspecified",
    }
}

/// Build a [`PartyRow`] from raw list fields (testable without a live server).
pub fn party_row_from_parts(
    id: impl Into<String>,
    display_name: impl Into<String>,
    role_values: &[i32],
) -> PartyRow {
    party_row_from_parts_active(id, display_name, role_values, true)
}

pub fn party_row_from_parts_active(
    id: impl Into<String>,
    display_name: impl Into<String>,
    role_values: &[i32],
    active: bool,
) -> PartyRow {
    let roles = role_values
        .iter()
        .copied()
        .map(party_role_label)
        .map(str::to_string)
        .collect();
    PartyRow {
        id: id.into(),
        display_name: display_name.into(),
        roles,
        active,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_labels_match_proto_numbers() {
        assert_eq!(party_role_label(0), "unspecified");
        assert_eq!(party_role_label(1), "customer");
        assert_eq!(party_role_label(2), "supplier");
        assert_eq!(party_role_label(3), "prospect");
        assert_eq!(party_role_label(99), "unspecified");
    }

    #[test]
    fn row_mapping_preserves_fields_without_inventing_rows() {
        let row = party_row_from_parts("id-1", "Acme", &[1, 2]);
        assert_eq!(row.id, "id-1");
        assert_eq!(row.display_name, "Acme");
        assert_eq!(row.roles, vec!["customer", "supplier"]);
    }

    #[test]
    fn empty_roles_stay_empty() {
        let row = party_row_from_parts("x", "Solo", &[]);
        assert!(row.roles.is_empty());
    }
}
