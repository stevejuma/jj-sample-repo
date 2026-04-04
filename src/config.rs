#[derive(Debug, Clone)]
pub struct AppConfig {
	pub environment: String,
}

impl AppConfig {
	pub fn load() -> Self {
		Self {
			environment: std::env::var("TASK_ENV").unwrap_or_else(|_| "local".to_string()),
		}
	}
}