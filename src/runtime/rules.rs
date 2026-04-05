use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRule {
    pub id: String,
    pub match_process_contains: String,
    pub workspace_id: String,
    pub launch_mode: String,
    pub pinned: bool,
}

impl AppRule {
    pub fn matches_process(&self, process_name: &str) -> bool {
        process_name
            .to_lowercase()
            .contains(&self.match_process_contains.to_lowercase())
    }
}

pub fn default_rules() -> Vec<AppRule> {
    vec![
        AppRule {
            id: "rule-dev-tools".to_string(),
            match_process_contains: "code".to_string(),
            workspace_id: "ws-2".to_string(),
            launch_mode: "tile".to_string(),
            pinned: true,
        },
        AppRule {
            id: "rule-browser".to_string(),
            match_process_contains: "chrome".to_string(),
            workspace_id: "ws-1".to_string(),
            launch_mode: "maximized".to_string(),
            pinned: true,
        },
        AppRule {
            id: "rule-chat".to_string(),
            match_process_contains: "discord".to_string(),
            workspace_id: "ws-3".to_string(),
            launch_mode: "float".to_string(),
            pinned: false,
        },
    ]
}
