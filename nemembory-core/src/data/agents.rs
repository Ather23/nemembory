use async_trait::async_trait;
use chrono::{Date, DateTime, Utc};
use sqlx::{PgPool, Pool, prelude::FromRow};

#[derive(serde::Serialize, FromRow, serde::Deserialize, Debug, Clone)]
pub struct Agent {
    pub id: i32,
    pub code: String,
    pub display_name: String,
    pub system_prompt: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

#[async_trait]
pub trait AgentPersistence {
    async fn save_agent(&self, agent: Agent) -> Result<(), DbError>;
    async fn load_agents(&self) -> Result<Vec<Agent>, DbError>;
    async fn get_agent(&self, id: i8) -> Result<Agent, DbError>;
    async fn add_agent(&self, agent: Agent) -> Result<Agent, DbError>;
}

pub struct DbAgentStore {
    pub db_pool: PgPool,
}

impl DbAgentStore {
    pub async fn new(conn_string: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            db_pool: PgPool::connect(conn_string).await?,
        })
    }
}

#[async_trait]
impl AgentPersistence for DbAgentStore {
    async fn save_agent(&self, agent: Agent) -> Result<(), DbError> {
        let query = "UPDATE agents SET display_name = $1, system_prompt = $2 WHERE code = $3";
        let code = &agent.code;
        sqlx::query(query)
            .bind(&agent.display_name)
            .bind(&agent.system_prompt)
            .bind(code)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn load_agents(&self) -> Result<Vec<Agent>, DbError> {
        let query = r#"SELECT id, code, display_name, system_prompt FROM agents"#;
        let agents = sqlx::query_as::<_, Agent>(query)
            .fetch_all(&self.db_pool)
            .await?;
        Ok(agents)
    }

    async fn get_agent(&self, id: i8) -> Result<Agent, DbError> {
        let query = "SELECT id, code, display_name, system_prompt FROM agents WHERE id = $1";
        let agent = sqlx::query_as::<_, Agent>(query)
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(agent)
    }

    async fn add_agent(&self, agent: Agent) -> Result<Agent, DbError> {
        let query = "INSERT INTO agents (code, display_name, system_prompt) VALUES ($1, $2, $3) RETURNING id, code, display_name, system_prompt";
        let new_agent = sqlx::query_as::<_, Agent>(query)
            .bind(&agent.code)
            .bind(&agent.display_name)
            .bind(&agent.system_prompt)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(new_agent)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Sql error: {0}")]
    SqlError(String),
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::SqlError(err.to_string())
    }
}
