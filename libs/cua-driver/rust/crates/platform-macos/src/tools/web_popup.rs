// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Cua AI, Inc.

/// Chromium exposes both a native select and an aria-haspopup button as
/// AXPopUpButton. An explicit "dialog" popup opens page content, not the native
/// select menu that raises the window. Require known non-menu children as well:
/// an aria-haspopup override on a real select must not disable its guard.
pub(crate) fn may_open_native_select(
    role: Option<&str>,
    popup_value: Option<&str>,
    known_non_menu_children: bool,
) -> bool {
    role == Some("AXPopUpButton") && !(popup_value == Some("dialog") && known_non_menu_children)
}

#[cfg(test)]
mod tests {
    use super::may_open_native_select;

    #[test]
    fn explicit_web_dialog_button_is_not_a_native_select() {
        assert!(!may_open_native_select(
            Some("AXPopUpButton"),
            Some("dialog"),
            true,
        ));
    }

    #[test]
    fn native_select_and_ambiguous_popup_values_keep_the_guard() {
        // Chromium's real select reports "menu". Unknown/missing values must
        // remain conservative; this is not a general menu-button exception.
        for value in [
            None,
            Some("menu"),
            Some("listbox"),
            Some("true"),
            Some(""),
            Some("unknown"),
        ] {
            assert!(
                may_open_native_select(Some("AXPopUpButton"), value, true),
                "{value:?}"
            );
        }
    }

    #[test]
    fn non_popup_roles_are_not_selects() {
        for role in [
            None,
            Some("AXButton"),
            Some("AXTextField"),
            Some("AXMenuItem"),
        ] {
            assert!(
                !may_open_native_select(role, Some("dialog"), true),
                "{role:?}"
            );
            assert!(
                !may_open_native_select(role, Some("menu"), true),
                "{role:?}"
            );
        }
    }

    #[test]
    fn dialog_metadata_without_known_non_menu_children_keeps_the_guard() {
        assert!(may_open_native_select(
            Some("AXPopUpButton"),
            Some("dialog"),
            false
        ));
    }
}
