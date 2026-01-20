// Copyright © 2026
// SPDX-License-Identifier: MIT

//! # Slint Radial Menu Library
//!
//! A reusable, data-driven radial menu system for Slint applications.
//!
//! ## Features
//!
//! - Hierarchical menus with unlimited nesting
//! - Mouse-direction based sub-menu triggering
//! - Option+click activation
//! - Minimal, professional visual style
//!
//! ## Usage
//!
//! ```rust,ignore
//! use slint_radial_menu::{MenuItem, MenuBuilder};
//!
//! let menu = MenuBuilder::new()
//!     .item("File")
//!         .child("New")
//!         .child("Open")
//!         .child("Save")
//!     .item("Edit")
//!         .child("Cut")
//!         .child("Copy")
//!         .child("Paste")
//!     .build();
//! ```

use slint::SharedString;

slint::include_modules!();

/// A menu item in the radial menu hierarchy.
/// Uses a flat structure with parent references for Slint compatibility.
#[derive(Clone, Debug)]
pub struct MenuItemData {
    pub id: i32,
    pub parent_id: i32,  // -1 for root items
    pub label: String,
    pub icon: Option<String>,  // Icon path or name
    pub enabled: bool,
}

impl MenuItemData {
    pub fn new(id: i32, parent_id: i32, label: impl Into<String>) -> Self {
        Self {
            id,
            parent_id,
            label: label.into(),
            icon: None,
            enabled: true,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Builder for creating menu hierarchies easily.
pub struct MenuBuilder {
    items: Vec<MenuItemData>,
    next_id: i32,
    current_parent: i32,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
            current_parent: -1,
        }
    }

    /// Add a root-level menu item.
    pub fn item(mut self, label: impl Into<String>) -> Self {
        let id = self.next_id;
        self.next_id += 1;
        self.current_parent = id;
        self.items.push(MenuItemData::new(id, -1, label));
        self
    }

    /// Add a child item to the current parent.
    pub fn child(mut self, label: impl Into<String>) -> Self {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(MenuItemData::new(id, self.current_parent, label));
        self
    }

    /// Add a child that becomes the new parent (for nesting).
    pub fn submenu(mut self, label: impl Into<String>) -> Self {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(MenuItemData::new(id, self.current_parent, label));
        self.current_parent = id;
        self
    }

    /// Go back up one level in the hierarchy.
    pub fn end_submenu(mut self) -> Self {
        if let Some(current) = self.items.iter().find(|i| i.id == self.current_parent) {
            self.current_parent = current.parent_id;
        }
        self
    }

    /// Build the menu items vector.
    pub fn build(self) -> Vec<MenuItemData> {
        self.items
    }
}

impl Default for MenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert MenuItemData to Slint's MenuItem struct.
pub fn to_slint_model(items: &[MenuItemData]) -> slint::ModelRc<MenuItem> {
    let slint_items: Vec<MenuItem> = items
        .iter()
        .map(|item| {
            // Count siblings and position for this item
            let siblings: Vec<_> = items
                .iter()
                .filter(|i| i.parent_id == item.parent_id)
                .collect();
            
            let sibling_count = siblings.len() as i32;
            let sibling_index = siblings
                .iter()
                .position(|i| i.id == item.id)
                .unwrap_or(0) as i32;
            
            MenuItem {
                id: item.id,
                parent_id: item.parent_id,
                label: SharedString::from(&item.label),
                icon: SharedString::from(item.icon.as_deref().unwrap_or("")),
                enabled: item.enabled,
                has_children: items.iter().any(|i| i.parent_id == item.id),
                sibling_index,
                sibling_count,
            }
        })
        .collect();

    std::rc::Rc::new(slint::VecModel::from(slint_items)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_builder() {
        let items = MenuBuilder::new()
            .item("File")
            .child("New")
            .child("Open")
            .item("Edit")
            .child("Cut")
            .build();

        assert_eq!(items.len(), 5);
        assert_eq!(items[0].parent_id, -1);  // File is root
        assert_eq!(items[1].parent_id, 1);   // New is child of File
    }
}
