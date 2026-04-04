#[derive(Debug, Clone)]
pub struct AppConfig {
	pub environment: String,
	pub command: Command,
	pub limit: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
	List,
	Owners,
}

impl AppConfig {
	pub fn load() -> Self {
		let mut environment = std::env::var("TASK_ENV").unwrap_or_else(|_| "local".to_string());
		let mut command = Command::List;
		let mut limit = None;
		let mut args = std::env::args().skip(1);

		while let Some(arg) = args.next() {
			match arg.as_str() {
				"owners" => command = Command::Owners,
				"list" => command = Command::List,
				"--env" => {
					if let Some(value) = args.next() {
						environment = value;
					}
				}
				"--limit" => {
					if let Some(value) = args.next() {
						limit = value.parse::<usize>().ok();
					}
				}
				_ => {}
			}
		}

		Self {
			environment,
			command,
			limit,
		}
	}

	pub fn validate(&self) -> Result<(), String> {
		if self.environment.trim().is_empty() {
			return Err("environment name cannot be empty".to_string());
		}

		if matches!(self.limit, Some(0)) {
			return Err("--limit must be greater than zero".to_string());
		}

		Ok(())
	}
}