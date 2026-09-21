use dnd_m8_core::{AppState, CharacterId};

/// State owned and managed by the Tauri shell.
pub struct ShellState {
    /// Ruleset-independent application state.
    core: AppState,

    /// Character currently selected by the application.
    active_character_id: CharacterId,
}

impl ShellState {
    /// Creates shell state with an initially active character.
    #[must_use]
    pub const fn new(core: AppState, active_character_id: CharacterId) -> Self {
        Self {
            core,
            active_character_id,
        }
    }

    /// Returns the core application state.
    #[must_use]
    pub const fn core(&self) -> &AppState {
        &self.core
    }

    /// Returns the currently active character identifier.
    #[must_use]
    pub const fn active_character_id(&self) -> CharacterId {
        self.active_character_id
    }
}
