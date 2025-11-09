#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub prompt: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

pub trait AgentPersistence {
    fn save_agent_to_file(&self, agent: &Agent);
    fn load_agents_from_file(&self) -> Vec<Agent>;
}

pub struct FileBasedAgentStore {
    path: String,
}

impl FileBasedAgentStore {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl AgentPersistence for FileBasedAgentStore {
    fn save_agent_to_file(&self, agent: &Agent) {
        let json = serde_json::to_string_pretty(agent).expect("Failed to serialize agent");
        std::fs::write(&self.path, json).expect("Failed to write agent.json");
    }

    fn load_agents_from_file(&self) -> Vec<Agent> {
        let file = std::fs::read_to_string(&self.path).expect("Failed to read agent.json");
        serde_json::from_str(&file).expect("Failed to parse agent.json")
    }
}
