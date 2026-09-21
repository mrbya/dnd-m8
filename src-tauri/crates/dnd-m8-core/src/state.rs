use crate::CharacterService;

/// Root state for the D7D Mate app core.
pub struct AppState {
    /// In-memory character service.
    characters: CharacterService,
}

impl AppState {
    /// Creates application state backed by the provided character service.
    #[must_use]
    pub const fn new(characters: CharacterService) -> Self {
        Self { characters }
    }

    /// Returns the character service.
    #[must_use]
    pub const fn characters(&self) -> &CharacterService {
        &self.characters
    }
}
