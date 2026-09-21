use dnd_m8_ruleset::{RulesetId, RulesetMetadata};

#[derive(Debug)]
struct TestRuleset {
    metadata: RulesetMetadata,
}

impl TestRuleset {
    pub fn new() -> Self {
        Self {
            metadata: RulesetMetadata {
                id: RulesetId::new("org.dnd-m8.test", "1.0.0").expect("valid id"),
                display_name: String::from("test ruleset"),
                description: String::from("kung foo"),
            },
        }
    }
}
