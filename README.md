# Hyprland-rs

[![Crates.io](https://img.shields.io/crates/v/hyprland)](https://crates.io/crates/hyprland)
![Crates.io](https://img.shields.io/crates/d/hyprland)
[![Crates.io](https://img.shields.io/crates/l/hyprland)](https://www.gnu.org/licenses/gpl-3.0.html)
[![docs.rs](https://img.shields.io/docsrs/hyprland)](https://docs.rs/hyprland)
[![Hyprland](https://img.shields.io/badge/Made%20for-Hyprland-blue)](https://github.com/hyprwm/Hyprland)
[![Discord](https://img.shields.io/discord/1055990214411169892?label=discord)](https://discord.gg/zzWqvcKRMy)

An unoffical rust wrapper for Hyprland's IPC

## Disclaimer
If something doesn't work, doesn't matter what,
make sure you are on the latest commit of Hyprland before making an issue!

## Getting started!

Lets get started with Hyprland-rs!

### Adding to your project

Add the code below to the dependencies section of your Cargo.toml file!

```toml
hyprland = "0.3.0"
```

### What this crate provides

This crate provides 3 modules (+1 for shared things)
 - `data` for getting information on the compositor
 - `event_listener` which provides the `EventListener` struct for listening for events
 - `dispatch` for calling dispatchers and changing keywords

## Example Usage

here is an example of most of the provided features being utilized

```rust ,no_run
use hyprland::data::{Client, Clients, Monitors};
use hyprland::dispatch::*;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::keyword::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;

fn main() -> hyprland::shared::HResult<()> {
    // We can call dispatchers with the dispatch macro, and struct!
    // You can decide what you want to use, below are some examples of their usage

    // Here we are telling hyprland to open kitty using the dispatch macro!
    hyprland::dispatch!(Exec, "kitty")?;

    // Here we are moving the cursor to the top left corner! We can also just use the Dispatch
    // struct!
    Dispatch::call(DispatchType::MoveCursorToCorner(Corner::TopLeft))?;

    // Here we are adding a keybinding to Hyprland using the bind macro!
    hyprland::bind!(SUPER, Key, "i" => ToggleFloating, None)?;

    // Here we are getting the border size
    let border_size = match Keyword::get("general:border_size")?.value {
        OptionValue::Int(i) => i,
        _ => panic!("border size can only be a int"),
    };
    println!("{border_size}");

    // Here we change a keyword, in this case we are doubling the border size we got above
    Keyword::set("general:border_size", border_size * 2)?;

    // get all monitors
    let monitors = Monitors::get()?;

    // and the active window
    let win = Client::get_active()?;

    // and all open windows
    let clients = Clients::get()?;

    // and printing them all out!
    println!("monitors: {monitors:#?},\nactive window: {win:#?},\nclients {clients:#?}");

    // Create a event listener
    let mut event_listener = EventListener::new();

    // This changes the workspace to 5 if the workspace is switched to 9
    // this is a performance and mutable state test
    event_listener.add_workspace_change_handler(|id, state| {
        if id == WorkspaceType::Regular('9'.to_string()) {
            state.active_workspace = WorkspaceType::Regular('2'.to_string());
        }
    });
    // This makes it so you can't turn on fullscreen lol
    event_listener.add_fullscreen_state_change_handler(|fstate, state| {
        if fstate {
            state.fullscreen_state = false;
        }
    });
    // Makes a monitor unfocusable
    event_listener.add_active_monitor_change_handler(|data, state| {
        let hyprland::event_listener::MonitorEventData(monitor, _) = data;

        if monitor == *"DP-1".to_string() {
            state.active_monitor = "eDP-1".to_string()
        }
    });

    // add event, yes functions and closures both work!
    event_listener.add_workspace_change_handler(|id, _| println!("workspace changed to {id:#?}"));
    // Waybar example
    // event_listener.add_active_window_change_handler(|data| {
    //     use hyprland::event_listener::WindowEventData;
    //     let string = match data {
    //         Some(WindowEventData(class, title)) => format!("{class}: {title}"),
    //         None => "".to_string()
    //     };
    //     println!(r#"{{"text": "{string}", class: "what is this?"}}"#);
    // });

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener()
}
```
