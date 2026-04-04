#[derive(Debug, Clone)]
pub struct AppConfig {
	pub environment: String,
	pub command: Command,
	pub deployment_target: Option<String>,
	pub limit: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
	List,
	Owners,
	DeployPreview,
}

impl AppConfig {
	pub fn load() -> Self {
		let mut environment = std::env::var("TASK_ENV").unwrap_or_else(|_| "local".to_string());
		let mut command = Command::List;
		let mut deployment_target = None;
		let mut limit = None;
		let mut args = std::env::args().skip(1);

		while let Some(arg) = args.next() {
			match arg.as_str() {
				"deploy-preview" => command = Command::DeployPreview,
				"owners" => command = Command::Owners,
				"list" => command = Command::List,
				"--env" => {
					if let Some(value) = args.next() {
						environment = value;
					}
				}
				"--deploy-target" => deployment_target = args.next(),
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
			deployment_target,
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