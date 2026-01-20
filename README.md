# Slint Radial Menu Library

A reusable, data-driven hierarchical radial menu system for [Slint](https://slint.dev) applications.

## Features

- **Hierarchical Menus**: Each item can have nested sub-menus
- **Data-Driven**: Define menus as flat arrays with parent references
- **Mouse-Direction Based**: Sub-menus appear based on mouse movement direction
- **Minimal Professional Style**: Clean, dark theme out of the box
- **Customizable**: Full control over colors, sizes, and animations
- **Option+Click Trigger**: Opens menu at cursor position

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
slint-radial-menu = { path = "../slint-radial-menu" }
```

## Quick Start

```rust
use slint_radial_menu::{MenuBuilder, to_slint_model};

// Build menu hierarchy with fluent API
let menu_items = MenuBuilder::new()
    .item("File")
        .child("New")
        .child("Open")
        .child("Save")
    .item("Edit")
        .child("Cut")
        .child("Copy")
        .child("Paste")
    .build();

// Convert to Slint model
let model = to_slint_model(&menu_items);
```

## Architecture

### Data Model

Uses a flat structure with parent references for Slint compatibility:

```rust
struct MenuItem {
    id: i32,
    parent_id: i32,  // -1 for root items
    label: String,
    icon: Option<String>,
    enabled: bool,
}
```

### Slint Components

- `RadialMenu`: Main component, handles display and interaction
- `RadialMenuItem`: Individual arc segment
- `RadialMenuConfig`: Customizable appearance settings

## Running the Demo

```bash
cargo run --example demo
```

## Customization

Override the default configuration:

```slint
RadialMenu {
    config: {
        inner-radius: 50px,
        outer-radius: 150px,
        item-color: #3d3d3d,
        item-hover-color: #5a5a5a,
        // ... other properties
    };
}
```

## License

MIT
