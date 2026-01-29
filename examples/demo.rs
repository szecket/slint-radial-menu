// Copyright © 2026
// SPDX-License-Identifier: MIT

//! Demo application for the radial menu library.

use slint::ComponentHandle;
use slint_radial_menu::MenuBuilder;

slint::slint! {
    import { RadialMenu, MenuItem } from "ui/radial-menu.slint";
    
    export component Demo inherits Window {
        title: "Radial Menu Demo";
        width: 800px;
        height: 600px;
        background: #2b2b2b;
        
        in property <[MenuItem]> menu-items: [];
        in-out property <int> menu-current-parent: -1;
        
        callback item-selected-callback(int);
        callback handle-radial-item(int);
        callback find-item-at-angle-callback(angle) -> int;
        callback show-submenu-callback(int, length, length);
        callback check-has-children-callback(int) -> bool;
        callback get-item-text-callback(int) -> string;
        callback get-item-parent-id-callback(int) -> int;
        
        // Background with a gradient and pattern to simulate an interface
        Rectangle {
            width: 100%;
            height: 100%;
            background: @linear-gradient(135deg, #1e3c72 0%, #2a5298 50%, #7e22ce 100%);
        }
        
        // Simulated UI elements
        Rectangle {
            x: 20px;
            y: 100px;
            width: 200px;
            height: 400px;
            background: #ffffff20;
            border-radius: 8px;
        }
        
        Rectangle {
            x: parent.width - 220px;
            y: 100px;
            width: 200px;
            height: 300px;
            background: #ffffff15;
            border-radius: 8px;
        }
        
        // Instructions
        Text {
            x: 20px;
            y: 20px;
            text: "Control+Click anywhere to open the radial menu";
            color: #ffffff;
            font-size: 14px;
        }
        
        Text {
            x: 20px;
            y: 45px;
            text: "Drag to select items. Hold over an item for 1 second to open submenu. Release to select.";
            color: #ffffffcc;
            font-size: 12px;
        }
        
        Text {
            x: 20px;
            y: 70px;
            text: "Press ESC to cancel.";
            color: #ffffffcc;
            font-size: 12px;
        }
        
        // Selected item display
        in-out property <string> selected-label: "None";
        Text {
            x: 20px;
            y: parent.height - 40px;
            text: "Last selected: " + selected-label;
            color: #ffffff;
            font-size: 14px;
        }
        
        // Full-window touch area for Control+click activation and mouse tracking
        Rectangle {
            // Transparent background makes Rectangle hittable for pointer events
            background: transparent;
            TouchArea {
                width: 100%;
                height: 100%;
                
                pointer-event(event) => {
                    if event.kind == PointerEventKind.down {
                        if event.modifiers.control {
                            debug("Command+click detected, opening menu at", self.mouse-x, self.mouse-y);
                            menu.open(self.mouse-x, self.mouse-y);
                        } else {
                            debug("Click without modifier");
                        }
                    } else if event.kind == PointerEventKind.up {
                        if menu.is-open() {
                            debug("Mouse released, closing menu");
                            menu.release();
                        }
                    }
                }
                
                moved => {
                    if menu.is-open() {
                        menu.update-mouse(self.mouse-x, self.mouse-y);
                    }
                }
            }
        }
        
        // The radial menu
        menu := RadialMenu {
            items: root.menu-items;
            current-parent-id <=> root.menu-current-parent;
            
            item-selected(id) => {
                root.item-selected-callback(id);
            }
            
            menu-closed => {
                debug("Menu closed");
            }
            
            handle-item-action(id) => {
                // This will be handled via Rust callback
                root.handle-radial-item(id);
            }
            
            find-item-at-angle(angle) => {
                root.find-item-at-angle-callback(angle)
            }
            
            show-submenu(item-id, mouse-x, mouse-y) => {
                root.show-submenu-callback(item-id, mouse-x, mouse-y);
            }
            
            check-item-has-children(item-id) => {
                root.check-has-children-callback(item-id)
            }
            
            get-item-text(item-id) => {
                root.get-item-text-callback(item-id)
            }
            
            get-item-parent-id(item-id) => {
                root.get-item-parent-id-callback(item-id)
            }
        }
        
        // Ghost text overlay - shows after selection
        if menu.ghost-active: Text {
            x: menu.ghost-x - self.width / 2;
            y: menu.ghost-y - self.height / 2;
            text: menu.ghost-text;
            font-size: 18px;
            font-weight: 600;
            color: #ffffff;
            opacity: menu.ghost-opacity;
        }
        
        // Keyboard handling for ESC
        FocusScope {
            key-pressed(event) => {
                if event.text == Key.Escape && menu.is-open() {
                    menu.close();
                    return accept;
                }
                return reject;
            }
        }
    }
}

fn main() {
    // Build a sample menu hierarchy
    let menu_items = MenuBuilder::new()
        .item("File")
            .child("New")
            .child("Open")
            .child("Save")
            .submenu("Recent")
                .child("Document 1")
                .child("Document 2")
                .child("Document 3")
            .end_submenu()
        .item("Edit")
            .child("Undo")
            .child("Redo")
            .child("Cut")
            .child("Copy")
            .child("Paste")
        .item("View")
            .child("Zoom In")
            .child("Zoom Out")
            .child("Fit to Screen")
        .item("Tools")
            .child("Settings")
            .child("Extensions")
        .item("Help")
            .child("Documentation")
            .child("About")
        .build();

    let demo = Demo::new().unwrap();
    
    // Convert MenuItemData to Demo::MenuItem (the struct type used by the demo)
    // First, compute sibling positions
    let slint_items: Vec<MenuItem> = menu_items
        .iter()
        .map(|item| {
            // Count siblings and position
            let siblings: Vec<_> = menu_items
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
                label: item.label.clone().into(),
                icon: item.icon.as_deref().unwrap_or("").into(),
                enabled: item.enabled,
                has_children: menu_items.iter().any(|i| i.parent_id == item.id),
                sibling_index,
                sibling_count,
            }
        })
        .collect();
    
    let model = std::rc::Rc::new(slint::VecModel::from(slint_items));
    demo.set_menu_items(model.into());
    
    // Find item at angle callback
    let demo_weak = demo.as_weak();
    let menu_items_clone = menu_items.clone();
    demo.on_find_item_at_angle_callback(move |angle| {
        let Some(demo) = demo_weak.upgrade() else { return -1 };
        
        // Get current parent from demo property
        let current_parent = demo.get_menu_current_parent();
        
        // Angle is passed from Slint in degrees (not radians!)
        // Normalize to 0-360 range
        let angle_norm = ((angle % 360.0) + 360.0) % 360.0;
        
        // Find matching item
        for item_data in menu_items_clone.iter() {
            if item_data.parent_id != current_parent {
                continue;
            }
            
            // Calculate this item's angle range
            let siblings: Vec<_> = menu_items_clone
                .iter()
                .filter(|i| i.parent_id == item_data.parent_id)
                .collect();
            
            let total = siblings.len() as f32;
            let index = siblings.iter().position(|i| i.id == item_data.id).unwrap_or(0) as f32;
            
            let angle_per_item = 360.0 / total;
            let gap = 2.0;  // Match config.item-gap-angle
            
            let start = -90.0 + index * angle_per_item + gap / 2.0;
            let sweep = angle_per_item - gap;
            let end = start + sweep;
            
            // Normalize start/end to 0-360
            let start_norm = if start < 0.0 { start + 360.0 } else { start } % 360.0;
            let end_norm = if end < 0.0 { end + 360.0 } else { end } % 360.0;
            
            // Check if angle falls in range, handling wraparound at 0°/360°
            let in_range = if start_norm <= end_norm {
                // Normal case: range doesn't cross 0°/360° boundary
                angle_norm >= start_norm && angle_norm <= end_norm
            } else {
                // Wraparound case: range crosses 0°/360° boundary
                angle_norm >= start_norm || angle_norm <= end_norm
            };
            
            if in_range {
                return item_data.id;
            }
        }
        
        -1  // No match
    });
    
    // Handle item action (selection or submenu) - this is called from the radial menu
    let demo_weak = demo.as_weak();
    let menu_items_for_action = menu_items.clone();
    demo.on_handle_radial_item(move |item_id| {
        let Some(demo) = demo_weak.upgrade() else { return };
        
        // Find the item
        let Some(item) = menu_items_for_action.iter().find(|i| i.id == item_id) else { return };
        
        // Check if it has children
        let has_children = menu_items_for_action.iter().any(|i| i.parent_id == item.id);
        
        if has_children {
            // This shouldn't happen anymore since we handle submenus via show-submenu callback
            // But keep for safety
            demo.set_menu_current_parent(item.id);
        } else {
            // Leaf item - select and close menu
            demo.invoke_item_selected_callback(item.id);
        }
    });
    
    // Check if item has children
    let menu_items_for_check = menu_items.clone();
    demo.on_check_has_children_callback(move |item_id| {
        menu_items_for_check.iter().any(|i| i.parent_id == item_id)
    });
    
    // Get item text for ghost display
    let menu_items_for_text = menu_items.clone();
    demo.on_get_item_text_callback(move |item_id| {
        menu_items_for_text
            .iter()
            .find(|i| i.id == item_id)
            .map(|i| i.label.clone().into())
            .unwrap_or_default()
    });

    // Get item parent-id
    let menu_items_for_parent = menu_items.clone();
    demo.on_get_item_parent_id_callback(move |item_id| {
        menu_items_for_parent
            .iter()
            .find(|i| i.id == item_id)
            .map(|i| i.parent_id)
            .unwrap_or(-1)
    });
    
    // Show submenu at mouse position
    let demo_weak = demo.as_weak();
    demo.on_show_submenu_callback(move |item_id, mouse_x, mouse_y| {
        let Some(demo) = demo_weak.upgrade() else { return };
        
        // Update the current parent to show submenu items
        demo.set_menu_current_parent(item_id);
    });
    
    // Handle item selection callback (for updating label)
    let demo_weak = demo.as_weak();
    let menu_items_for_callback = menu_items.clone();
    demo.on_item_selected_callback(move |id| {
        let demo = demo_weak.upgrade().unwrap();
        if let Some(item) = menu_items_for_callback.iter().find(|i| i.id == id) {
            println!("Selected: {} (id: {})", item.label, id);
            demo.set_selected_label(item.label.clone().into());
        } else {
            println!("Selected unknown item with id: {}", id);
        }
    });
    
    demo.run().unwrap();
}