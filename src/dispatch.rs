//! # Dispatch module
//!
//! This module is used for calling dispatchers and changing keywords
//!
//! ## Usage
//!
//! ```rust
//! use hyprland::shared::HResult;
//! use hyprland::dispatch::{Dispatch, DispatchType};
//! fn main() -> HResult<()> {
//!    Dispatch::call(DispatchType::Exec("kitty"))?;
//!
//!    Ok(())
//! }
//! ````

use crate::shared::*;
use std::string::ToString;
use strum_macros::Display;

/// This enum is for identifying a window
#[derive(Debug, Clone)]
pub enum WindowIdentifier<'a> {
    /// The address of a window
    Address(Address),
    /// A Regular Expression to match the window class (handled by Hyprland)
    ClassRegularExpression(&'a str),
    /// The window title
    Title(&'a str),
    /// The window's process Id
    ProcessId(u32),
}

impl std::fmt::Display for WindowIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            WindowIdentifier::Address(addr) => format!("address:{addr}"),
            WindowIdentifier::ProcessId(id) => format!("pid:{id}"),
            WindowIdentifier::ClassRegularExpression(regex) => regex.to_string(),
            WindowIdentifier::Title(title) => format!("title:{title}"),
        };
        write!(f, "{out}")
    }
}

/// This enum holds the fullscreen types
#[derive(Debug, Clone, Display)]
pub enum FullscreenType {
    /// Fills the whole screen
    #[strum(serialize = "0")]
    Real,
    /// Maximizes the window
    #[strum(serialize = "1")]
    Maximize,
    /// Passes no param
    #[strum(serialize = "")]
    NoParam,
}

/// This enum holds directions, typically used for moving
#[derive(Debug, Clone, Display)]
#[allow(missing_docs)]
pub enum Direction {
    #[strum(serialize = "u")]
    Up,
    #[strum(serialize = "d")]
    Down,
    #[strum(serialize = "r")]
    Right,
    #[strum(serialize = "l")]
    Left,
}

/// This enum is used for resizing and moving windows precisely
#[derive(Debug, Clone)]
pub enum Position {
    /// A delta
    Delta(i16, i16),
    /// The exact size
    Exact(i16, i16),
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Position::Delta(x, y) => format!("{x},{y}"),
            Position::Exact(w, h) => format!("exact {w} {h}"),
        };
        write!(f, "{out}")
    }
}

/// This enum holds a direction for cycling
#[allow(missing_docs)]
#[derive(Debug, Clone, Display)]
pub enum CycleDirection {
    #[strum(serialize = "")]
    Next,
    #[strum(serialize = "prev")]
    Previous,
}

/// This enum is used for identifying monitors
#[derive(Debug, Clone)]
pub enum MonitorIdentifier<'a> {
    /// The monitor that is to the specified direction of the active one
    Direction(Direction),
    /// The monitor id
    Id(u8),
    /// The monitor name
    Name(&'a str),
    /// The current monitor
    Current,
    /// The workspace relative to the current workspace
    Relative(i32),
}

impl std::fmt::Display for MonitorIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            MonitorIdentifier::Direction(dir) => dir.to_string(),
            MonitorIdentifier::Id(id) => id.to_string(),
            MonitorIdentifier::Name(name) => name.to_string(),
            MonitorIdentifier::Current => "current".to_string(),
            MonitorIdentifier::Relative(int) => format_relative(*int, ""),
        };
        write!(f, "{out}")
    }
}

/// This enum holds corners
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Corner {
    TopRight = 0,
    TopLeft = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

/// This enum holds options that are applied to the current workspace
#[derive(Debug, Clone, Display)]
pub enum WorkspaceOptions {
    /// Makes all windows pseudo tiled
    #[strum(serialize = "allfloat")]
    AllPseudo,
    /// Makes all windows float
    #[strum(serialize = "allpseudo")]
    AllFloat,
}

/// This enum is for identifying workspaces that also includes the special workspace
#[derive(Debug, Clone)]
pub enum WorkspaceIdentifierWithSpecial<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    Relative(i32),
    /// The workspace on the monitor relative to the current monitor
    RelativeMonitor(i32),
    /// The open workspace relative to the current workspace
    RelativeOpen(i32),
    /// The previous Workspace
    Previous,
    /// The first available empty workspace
    Empty,
    /// The name of the workspace
    Name(&'a str),
    /// The special workspace
    Special(Option<&'a str>),
}

impl std::fmt::Display for WorkspaceIdentifierWithSpecial<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WorkspaceIdentifierWithSpecial::*;
        let out = match self {
            Id(id) => format!("{id}"),
            Name(name) => format!("name:{name}"),
            Relative(int) => format_relative(*int, ""),
            RelativeMonitor(int) => format_relative(*int, "m"),
            RelativeOpen(int) => format_relative(*int, "e"),
            Previous => "previous".to_string(),
            Empty => "empty".to_string(),
            Special(opt) => match opt {
                Some(name) => format!("special:{name}"),
                None => "special".to_string(),
            },
        };

        write!(f, "{out}")
    }
}

/// This enum is for identifying workspaces
#[derive(Debug, Clone)]
pub enum WorkspaceIdentifier<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    Relative(i32),
    /// The workspace on the monitor relative to the current monitor
    RelativeMonitor(i32),
    /// The open workspace relative to the current workspace
    RelativeOpen(i32),
    /// The previous Workspace
    Previous,
    /// The first available empty workspace
    Empty,
    /// The name of the workspace
    Name(&'a str),
}

impl std::fmt::Display for WorkspaceIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WorkspaceIdentifier::*;
        let out = match self {
            Id(id) => format!("{id}"),
            Name(name) => format!("name:{name}"),
            Relative(int) => format_relative(*int, ""),
            RelativeMonitor(int) => format_relative(*int, "m"),
            RelativeOpen(int) => format_relative(*int, "e"),
            Previous => "previous".to_string(),
            Empty => "empty".to_string(),
        };

        write!(f, "{out}")
    }
}

/// This enum is the params to MoveWindow dispatcher
#[derive(Debug, Clone)]
pub enum WindowMove<'a> {
    /// Moves the window to a specified monitor
    Monitor(MonitorIdentifier<'a>),
    /// Moves the window in a specified direction
    Direction(Direction),
}

/// This enum holds every dispatcher
#[derive(Debug, Clone)]
pub enum DispatchType<'a> {
    /// This dispatcher changes the current cursor
    SetCursor(
        /// The cursor theme
        &'a str,
        /// The size
        u16,
    ),
    /// This dispatcher executes a program
    Exec(&'a str),
    /// This dispatcher passes a keybind to a window when called in a
    /// keybind, its used for global keybinds. And should **ONLY** be used with keybinds
    Pass(WindowIdentifier<'a>),
    /// This dispatcher kills the active window/client
    KillActiveWindow,
    /// This dispatcher closes the specified window
    CloseWindow(WindowIdentifier<'a>),
    /// This dispatcher changes the current workspace
    Workspace(WorkspaceIdentifierWithSpecial<'a>),
    /// This dispatcher moves a window (focused if not specified) to a workspace
    MoveToWorkspace(WorkspaceIdentifier<'a>, Option<WindowIdentifier<'a>>),
    /// This dispatcher moves a window (focused if not specified) to a workspace, without switching to that
    /// workspace
    MoveToWorkspaceSilent(WorkspaceIdentifier<'a>, Option<WindowIdentifier<'a>>),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// changes the active workspace aswell
    MoveFocusedWindowToWorkspace(WorkspaceIdentifier<'a>),
    /// This dispatcher moves the focused window to a specified workspace, and
    /// does not change workspaces
    MoveFocusedWindowToWorkspaceSilent(WorkspaceIdentifier<'a>),
    /// This dispatcher floats a window (current if not specified)
    ToggleFloating(Option<WindowIdentifier<'a>>),
    /// This dispatcher toggles the current window fullscreen state
    ToggleFullscreen(FullscreenType),
    /// This dispatcher toggles the focused window’s internal
    /// fullscreen state without altering the geometry
    ToggleFakeFullscreen,
    /// This dispatcher sets the DPMS status for all monitors
    ToggleDPMS(bool, Option<&'a str>),
    /// This dispatcher toggles pseudo tiling for the current window
    TogglePseudo,
    /// This dispatcher pins the active window to all workspaces
    TogglePin,
    /// This dispatcher moves the window focus in a specified direction
    MoveFocus(Direction),
    /// This dispatcher moves the current window to a monitor or in a specified direction
    MoveWindow(WindowMove<'a>),
    /// This dispatcher centers the active window
    CenterWindow,
    /// This dispatcher resizes the active window using a [`Position`][Position] enum
    ResizeActive(Position),
    /// This dispatcher moves the active window using a [`Position`][Position] enum
    MoveActive(Position),
    /// This dispatcher resizes the specified window using a [`Position`][Position] enum
    ResizeWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher moves the specified window using a [`Position`][Position] enum
    MoveWindowPixel(Position, WindowIdentifier<'a>),
    /// This dispatcher cycles windows using a specified direction
    CycleWindow(CycleDirection),
    /// This dispatcher swaps windows using a specified direction
    SwapWindow(CycleDirection),
    /// This dispatcher focuses a specified window
    FocusWindow(WindowIdentifier<'a>),
    /// This dispatcher focuses a specified monitor
    FocusMonitor(MonitorIdentifier<'a>),
    /// This dispatcher changed the split ratio
    ChangeSplitRatio(f32),
    /// This dispatcher toggle opacity for the current window/client
    ToggleOpaque,
    /// This dispatcher moves the cursor to a specified corner of a window
    MoveCursorToCorner(Corner),
    /// This dispatcher applied a option to all windows in a workspace
    WorkspaceOption(WorkspaceOptions),
    /// This dispatcher renames a workspace
    RenameWorkspace(WorkspaceId, Option<&'a str>),
    /// This exits Hyprland **(DANGEROUS)**
    Exit,
    /// This dispatcher forces the renderer to reload
    ForceRendererReload,
    /// This dispatcher moves the current workspace to a specified monitor
    MoveCurrentWorkspaceToMonitor(MonitorIdentifier<'a>),
    /// This dispatcher moves a specified workspace to a specified monitor
    MoveWorkspaceToMonitor(WorkspaceIdentifier<'a>, MonitorIdentifier<'a>),
    /// This dispatcher swaps the active workspaces of two monitors
    SwapActiveWorkspaces(MonitorIdentifier<'a>, MonitorIdentifier<'a>),
    /// This dispatcher brings the active window to the top of the stack
    BringActiveToTop,
    /// This toggles the special workspace (AKA scratchpad)
    ToggleSpecialWorkspace,
    /// This dispatcher jump to urgent or the last window
    FocusUrgentOrLast,
}

fn format_relative<T: Ord + std::fmt::Display + num_traits::Signed>(
    int: T,
    extra: &'_ str,
) -> String {
    if int.is_positive() {
        format!("{extra}+{int}")
    } else if int.is_negative() {
        format!("{extra}-{int}", int = int.abs())
    } else {
        "+0".to_string()
    }
}

pub(crate) fn gen_dispatch_str(cmd: DispatchType, dispatch: bool) -> HResult<String> {
    use DispatchType::*;
    let sep = if dispatch { " " } else { "," };
    let string_to_pass = match &cmd {
        Exec(sh) => format!("exec{sep}{sh}"),
        Pass(win) => format!("pass{sep}{win}"),
        KillActiveWindow => "killactive".to_string(),
        CloseWindow(win) => {
            format!("closewindow{sep}{win}")
        }
        Workspace(work) => format!("workspace{sep}{work}"),
        MoveToWorkspace(work, Some(win)) => format!("movetoworkspace {work} {win}"),
        MoveToWorkspace(work, None) => format!("movetoworkspace {work}"),
        MoveToWorkspaceSilent(work, Some(win)) => format!("movetoworkspacesilent {work} {win}"),
        MoveToWorkspaceSilent(work, None) => format!("movetoworkspacesilent {work}"),
        MoveFocusedWindowToWorkspace(work) => {
            format!("workspace{sep}{work}",)
        }
        MoveFocusedWindowToWorkspaceSilent(work) => {
            format!("workspace{sep}{work}",)
        }
        ToggleFloating(Some(v)) => format!("togglefloating{sep}{v}"),
        ToggleFloating(None) => "togglefloating".to_string(),
        ToggleFullscreen(ftype) => format!("fullscreen{sep}{ftype}"),
        ToggleFakeFullscreen => "fakefullscreen".to_string(),
        ToggleDPMS(stat, mon) => {
            format!(
                "dpms{sep}{} {}",
                if *stat { "on" } else { "off" },
                mon.unwrap_or_default()
            )
        }
        TogglePseudo => "pseudo".to_string(),
        TogglePin => "pin".to_string(),
        MoveFocus(dir) => format!("movefocus{sep}{dir}",),
        MoveWindow(ident) => format!(
            "movewindow{sep}{}",
            match ident {
                WindowMove::Direction(dir) => dir.to_string(),
                WindowMove::Monitor(mon) => format!("mon:{mon}"),
            }
        ),
        CenterWindow => "centerwindow".to_string(),
        ResizeActive(pos) => {
            format!("resizeactive{sep}{pos}")
        }
        MoveActive(pos) => format!("moveactive {pos}"),
        ResizeWindowPixel(pos, win) => {
            format!("resizeactive{sep}{pos} {win}")
        }
        MoveWindowPixel(pos, win) => {
            format!("moveactive{sep}{pos} {win}")
        }
        CycleWindow(dir) => format!("cyclenext{sep}{dir}"),
        SwapWindow(dir) => format!("swapnext{sep}{dir}"),
        FocusWindow(win) => format!("focuswindow{sep}{win}"),
        FocusMonitor(mon) => format!("focusmonitor{sep}{mon}"),
        ChangeSplitRatio(ratio) => format!("splitratio {ratio}"),
        ToggleOpaque => "toggleopaque".to_string(),
        MoveCursorToCorner(corner) => format!("movecursortocorner{sep}{}", corner.clone() as u8),

        WorkspaceOption(opt) => format!("workspaceopt{sep}{opt}"),
        Exit => "exit".to_string(),
        ForceRendererReload => "forcerendererreload".to_string(),
        MoveCurrentWorkspaceToMonitor(mon) => {
            format!("movecurrentworkspacetomonitor{sep}{mon}")
        }
        MoveWorkspaceToMonitor(work, mon) => {
            format!("moveworkspacetomonitor{sep}{work} {mon}",)
        }
        ToggleSpecialWorkspace => "togglespecialworkspace".to_string(),
        RenameWorkspace(id, name) => {
            format!(
                "renameworkspace{sep}{id} {}",
                name.unwrap_or(&id.to_string())
            )
        }
        SwapActiveWorkspaces(mon, mon2) => format!("swapactiveworkspaces{sep}{mon} {mon2}",),
        BringActiveToTop => "bringactivetotop".to_string(),
        SetCursor(theme, size) => {
            format!("{theme} {}", *size)
        }
        FocusUrgentOrLast => "focusurgentorlast".to_string(),
    };
    if let SetCursor(_, _) = cmd {
        Ok(format!("setcursor {string_to_pass}"))
    } else if dispatch {
        Ok(format!("dispatch {string_to_pass}"))
    } else {
        Ok(string_to_pass)
    }
}

/// The struct that provides all dispatching methods
pub struct Dispatch;

impl Dispatch {
    /// This function calls a specified dispatcher (blocking)
    ///
    /// ```rust
    /// # use hyprland::shared::HResult;
    /// # fn main() -> HResult<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call(DispatchType::Exec("kitty"))
    /// # }
    /// ```
    pub fn call(dispatch_type: DispatchType) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let output = write_to_socket_sync(
            socket_path,
            gen_dispatch_str(dispatch_type, true)?.as_bytes(),
        );

        match output {
            Ok(msg) => match msg.as_str() {
                "ok" => Ok(()),
                msg => Err(HyprError::NotOkDispatch(msg.to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// This function calls a specified dispatcher (async)
    ///
    /// ```rust
    /// # use hyprland::shared::HResult;
    /// # async fn function() -> HResult<()> {
    /// use hyprland::dispatch::{DispatchType,Dispatch};
    /// // This is an example of just one dispatcher, there are many more!
    /// Dispatch::call_async(DispatchType::Exec("kitty")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_async(dispatch_type: DispatchType<'_>) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let output = write_to_socket(
            socket_path,
            gen_dispatch_str(dispatch_type, true)?.as_bytes(),
        )
        .await;

        match output {
            Ok(msg) => match msg.as_str() {
                "ok" => Ok(()),
                msg => Err(HyprError::NotOkDispatch(msg.to_string())),
            },
            Err(error) => Err(error),
        }
    }
}

/// Macro abstraction over [Dispatch::call]
#[macro_export]
macro_rules! dispatch {
    ($dis:ident, $( $arg:expr ), *) => {
        Dispatch::call(DispatchType::$dis($($arg), *))
    };
    (async; $dis:ident, $( $arg:expr ), *) => {
        Dispatch::call_async(DispatchType::$dis($($arg), *))
    };
}
