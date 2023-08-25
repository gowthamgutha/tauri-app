# Svelte + Vite

This template should help get you started developing with Tauri and Svelte in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

# Directory structure

1. `src-tauri` -- (Backend) This contains the Rust programs that are used for backend functionality.

2. `src` -- (Frontend) This contains the svelte files that is used for the frontend.


# A brief on the Svelte, Tauri and Rust

In svelte, a component is a conglomeration of HTML, CSS and JavaScript where HTMl provides the structure of the component, CSS provides its styling and JavaScript defines its functionality.

Rust is used as backend which means it can be used to implement functionality that relates to the backend. We can perform network calls, access the operating system etc.,

Tauri provides a bridge between the frontend application written in Svelte and the backend application written in Rust.

Vite is a build tool used for building frontend applications, including this Tauri application. As part of the build, both the Rust programs in `src-tauri` and the Svelte files written in `src` are built, depending on what has changed.

Rust is built using its own tool called cargo which is merely called as part of the build process.
