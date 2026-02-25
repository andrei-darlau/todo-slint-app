# Rust + Slint TODO App

A simple **TODO list desktop application** built with **Rust** and [Slint](https://slint.dev/), demonstrating reactive UI programming, model-view binding, and basic user interactions.

---

## Features

- Add new TODO items with a title.
- Mark TODO items as checked (using checkboxes).
- Clear all TODO items at once.
- Reactive UI updates via `VecModel` binding.
- Minimal and clean design using `VerticalBox`, `HorizontalBox`, and `ListView`.

---

## Project Structure

- **`main.rs`** – Rust application logic:
  - Creates the main window (`AppWindow`).
  - Initializes a `VecModel` to store TODO items.
  - Handles `add_item` and `clear_all` callbacks.
- **`std-widgets.slint`** – Slint UI definition:
  - Defines the `AppWindow` component.
  - Uses `VerticalBox`/`HorizontalBox` for layout.
  - Contains a `LineEdit` for input, `Button`s for adding/clearing, and a `ListView` for TODO items.
  - Defines `TodoItem` struct for binding to the model.

---

## Running the App

1. Make sure you have Rust installed ([rustup](https://rustup.rs/)) and the `slint` Rust crate:

```bash
cargo install slint
```

2. Clone the project:

```bash
git clone <your-repo-url>
cd <project-folder>
```

3. Build and run:

```bash
cargo run
```

---

## How it Works

1. The main Rust code creates a `VecModel` (`todo_model`) to hold the list of TODO items.
2. The `AppWindow` component binds to `todo_model` using an `in-out property`.
3. Adding an item:
   - User types a title in the input field and clicks `+`.
   - The `add_item` callback is called.
   - Rust pushes a new `TodoItem` into the `VecModel`.
4. Clearing all items:
   - Clicking "clear list" triggers the `clear_all` callback.
   - Rust clears the `VecModel`, and the UI updates automatically.
5. Each TODO item is displayed with a checkbox reflecting its `checked` state.

---

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [Slint](https://slint.dev/) (`slint` crate)
- Standard Rust library (`std`)

---

## Notes

- This is a minimal demonstration of **reactive UI** with Slint in Rust.
- `VecModel` allows two-way binding between the Rust backend and the UI.
- The project can be extended with features like:
  - Persistent storage (saving TODOs to a file)
  - Editing item titles
  - Removing individual items

---
