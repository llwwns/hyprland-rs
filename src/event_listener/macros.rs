// macro_rules! gen_event_adder {
//     ($name:literal,$str:tt) => {
//         format!("listener.{}(|data| println!({}))", $name, $str)
//     };
// }

macro_rules! add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name $end _events>].push(EventTypes::Regular(Box::new(f)));
                }
            }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#"| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f) + 'static) {
                    self.events.[<$name _events>].push(EventTypes::Regular(Box::new(f)));
                }
            }
        }
    };
}

macro_rules! mut_add_listener {
    ($name:ident $end:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            doc_comment! { concat!("This methods adds a event which ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListenerMutable as EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#", _| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut State) + 'static) {
                    self.events.[<$name $end _events>].push(EventTypes::MutableState(Box::new(f)));
                }
            }
        }
    };
    ($name:ident,$f:ty,$c:literal,$c2:expr => $id:ident) => {
        paste! {
            doc_comment! { concat!("This methods adds a event which executes when ", $c, r#"
```rust, no_run
use hyprland::event_listener::EventListenerMutable as EventListener;
let mut listener = EventListener::new();
listener.add_"#, stringify!($name), r#"_handler(|"#, stringify!($id), r#", _| println!(""#, $c2, ": {", stringify!($id), r#":#?}"));
listener.start_listener();"#),
                pub fn [<add_ $name _handler>](&mut self, f: impl Fn($f, &mut State) + 'static) {
                    self.events.[<$name _events>].push(EventTypes::MutableState(Box::new(f)));
                }
            }
        }
    };
}

macro_rules! mut_arm {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for i in events.iter() {
            let new_state = execute_closure_mut($se.state.clone(), i, $val).await?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_state_arm {
    ($val:expr,$nam:ident,$na:ident,$va:expr,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na = $va;
        for i in events.iter() {
            let new_state = execute_closure_mut($se.state.clone(), i, $val).await?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_arm_sync {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for i in events.iter() {
            let new_state = execute_closure_mut_sync($se.state.clone(), i, $val)?;
            $se.state = new_state;
        }
    }};
}

macro_rules! mut_state_arm_sync {
    ($val:expr,$nam:ident,$na:ident,$va:expr,$se:ident) => {{
        let events = &$se.events.$nam;
        $se.state.$na = $va;
        for i in events.iter() {
            let new_state = execute_closure_mut_sync($se.state.clone(), i, $val)?;
            $se.state = new_state;
        }
    }};
}

macro_rules! arm {
    ($val:expr,$nam:ident,$se:ident) => {{
        let events = &$se.events.$nam;
        for item in events.iter() {
            execute_closure(item, $val);
        }
    }};
}

macro_rules! init_events {
    () => {
        Events {
            workspace_changed_events: vec![],
            workspace_added_events: vec![],
            workspace_destroyed_events: vec![],
            workspace_moved_events: vec![],
            active_monitor_changed_events: vec![],
            active_window_changed_events: vec![],
            fullscreen_state_changed_events: vec![],
            monitor_removed_events: vec![],
            monitor_added_events: vec![],
            window_open_events: vec![],
            window_close_events: vec![],
            window_moved_events: vec![],
            keyboard_layout_change_events: vec![],
            sub_map_changed_events: vec![],
            layer_open_events: vec![],
            layer_closed_events: vec![],
            float_state_events: vec![],
            urgent_state_events: vec![],
        }
    };
}
