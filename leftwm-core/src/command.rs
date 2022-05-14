use crate::models::{TagId, WindowHandle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Command {
    Execute(String),
    CloseWindow,
    SwapScreens,
    SoftReload,
    HardReload,
    ToggleScratchPad(String),
    ToggleFullScreen,
    ToggleSticky,
    GoToTag {
        tag: TagId,
        swap: bool,
    },
    ReturnToLastTag,
    FloatingToTile,
    TileToFloating,
    ToggleFloating,

    FocusNextTag,
    FocusPreviousTag,
    FocusWindow(String),
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    SendWindowToTag {
        window: Option<WindowHandle>,
        tag: TagId,
    },
    MoveWindowToLastWorkspace,
    MoveWindowToNextWorkspace,
    MoveWindowToPreviousWorkspace,
    MouseMoveWindow,
    SetMarginMultiplier(f32),
    SendWorkspaceToTag(usize, usize),
    CloseAllOtherWindows,
    Other(String),
}
