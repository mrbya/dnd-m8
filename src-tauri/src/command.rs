use tauri::State;

use crate::{dto::CharacterSummaryDto, error::CommandResult, state::ShellState};

/// Returns the currently active character.
///
/// # Errors
///
/// Returns an error when the character cannot be summarized.
#[tauri::command]
pub fn get_character(state: State<'_, ShellState>) -> CommandResult<CharacterSummaryDto> {
    let inner_state = state;
    get_character_from_state(inner_state.inner())
}

/// Implements [`get_character`] without depending on Tauri's state wrapper.
///
/// Keeping this operation separate makes the command boundary unit-testable
/// without constructing a Tauri runtime.
fn get_character_from_state(state: &ShellState) -> CommandResult<CharacterSummaryDto> {
    let character_id = state.active_character_id();

    let summary = state
        .core()
        .characters()
        .summarize_character(character_id)?;

    Ok(CharacterSummaryDto::from_domain(character_id, summary))
}

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;

    use pretty_assertions::assert_eq;

    use crate::{bootstrap::build_state, command::get_character_from_state};

    /// Result used by fallible command tests.
    type TestResult = Result<(), Box<dyn StdError>>;

    #[test]
    fn get_character_returns_active_demo_character() -> TestResult {
        let state = build_state()?;

        let character = get_character_from_state(&state)?;

        assert_eq!(character.name, "Mira Ashfall");
        assert_eq!(character.level, 5);
        assert_eq!(character.class_name, "Life Domain Cleric");
        assert_eq!(character.current_hit_points, 31);
        assert_eq!(character.max_hit_points, 38);
        assert_eq!(character.armor_class, 18);
        assert_eq!(character.proficiency_bonus, 3);
        assert_eq!(character.speed, 30);
        assert!(!character.id.is_empty());

        Ok(())
    }
}
