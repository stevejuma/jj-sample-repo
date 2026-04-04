use crate::{config::AppConfig, tasks::Task};

pub fn render_header(config: &AppConfig) -> String {
	format!("Task Tracker [{}]", config.environment)
}

pub fn render_tasks(tasks: &[Task]) -> String {
	let mut lines = Vec::new();
	for task in tasks {
		lines.push(format!("- {} ({})", task.title, task.status));
	}
	lines.join("\n")
}