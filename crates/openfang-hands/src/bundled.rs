//! Compile-time embedded Hand definitions.

use crate::{parse_hand_toml, HandDefinition, HandError};

/// Returns all bundled hand definitions as (id, HAND.toml content, SKILL.md content).
pub fn bundled_hands() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![]
}

/// Parse a bundled HAND.toml into a HandDefinition with its skill content attached.
pub fn parse_bundled(
    _id: &str,
    toml_content: &str,
    skill_content: &str,
) -> Result<HandDefinition, HandError> {
    let mut def: HandDefinition =
        parse_hand_toml(toml_content).map_err(|e| HandError::TomlParse(e.to_string()))?;
    if !skill_content.is_empty() {
        def.skill_content = Some(skill_content.to_string());
    }
    Ok(def)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bundled_hands_parse() {
        for (id, toml_content, skill_content) in bundled_hands() {
            let def = parse_bundled(id, toml_content, skill_content)
                .unwrap_or_else(|e| panic!("Failed to parse hand '{}': {}", id, e));
            assert_eq!(def.id, id);
            assert!(!def.name.is_empty());
            assert!(!def.tools.is_empty());
            assert!(!def.agent.system_prompt.is_empty());
            assert!(def.skill_content.is_some());
        }
    }

    #[test]
    fn all_einstein_hands_have_schedules() {
        let einstein_ids = [
            "lead",
            "collector",
            "predictor",
            "researcher",
            "twitter",
            "trader",
        ];
        for (id, toml_content, skill_content) in bundled_hands() {
            if einstein_ids.contains(&id) {
                let def = parse_bundled(id, toml_content, skill_content).unwrap();
                assert!(
                    def.tools.contains(&"schedule_create".to_string()),
                    "Einstein hand '{}' must have schedule_create tool",
                    id
                );
                assert!(
                    def.tools.contains(&"schedule_list".to_string()),
                    "Einstein hand '{}' must have schedule_list tool",
                    id
                );
                assert!(
                    def.tools.contains(&"schedule_delete".to_string()),
                    "Einstein hand '{}' must have schedule_delete tool",
                    id
                );
            }
        }
    }

    #[test]
    fn all_einstein_hands_have_memory() {
        let einstein_ids = [
            "lead",
            "collector",
            "predictor",
            "researcher",
            "twitter",
            "trader",
        ];
        for (id, toml_content, skill_content) in bundled_hands() {
            if einstein_ids.contains(&id) {
                let def = parse_bundled(id, toml_content, skill_content).unwrap();
                assert!(
                    def.tools.contains(&"memory_store".to_string()),
                    "Einstein hand '{}' must have memory_store tool",
                    id
                );
                assert!(
                    def.tools.contains(&"memory_recall".to_string()),
                    "Einstein hand '{}' must have memory_recall tool",
                    id
                );
            }
        }
    }

    #[test]
    fn all_einstein_hands_have_knowledge_graph() {
        let einstein_ids = [
            "lead",
            "collector",
            "predictor",
            "researcher",
            "twitter",
            "trader",
        ];
        for (id, toml_content, skill_content) in bundled_hands() {
            if einstein_ids.contains(&id) {
                let def = parse_bundled(id, toml_content, skill_content).unwrap();
                assert!(
                    def.tools.contains(&"knowledge_add_entity".to_string()),
                    "Einstein hand '{}' must have knowledge_add_entity tool",
                    id
                );
                assert!(
                    def.tools.contains(&"knowledge_add_relation".to_string()),
                    "Einstein hand '{}' must have knowledge_add_relation tool",
                    id
                );
                assert!(
                    def.tools.contains(&"knowledge_query".to_string()),
                    "Einstein hand '{}' must have knowledge_query tool",
                    id
                );
            }
        }
    }
}
