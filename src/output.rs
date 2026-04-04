use crate::{config::AppConfig, tasks::Task};

use crate::cloud::DeploymentPlan;

pub fn render_header(config: &AppConfig) -> String {
	match config.deployment_target.as_deref() {
		Some(target) => format!("Task Tracker [{}] -> deploy {target}", config.environment),
		None => format!("Task Tracker [{}]", config.environment),
	}
}

pub fn render_summary(tasks: &[Task]) -> String {
	let total = tasks.len();
	let todo = tasks.iter().filter(|task| task.status == "todo").count();
	let in_progress = tasks
		.iter()
		.filter(|task| task.status == "in-progress")
		.count();
	let blocked = tasks.iter().filter(|task| task.status == "blocked").count();

	format!(
		"{} tasks | {} todo | {} in-progress | {} blocked",
		total, todo, in_progress, blocked
	)
}

pub fn render_tasks(tasks: &[Task]) -> String {
	let mut lines = Vec::new();
	for task in tasks {
		lines.push(format!(
			"- #{} {} [{}] owner={} due={}",
			task.id, task.title, task.status, task.owner, task.due_date
		));
	}
	lines.join("\n")
}

pub fn render_owners(tasks: &[Task]) -> String {
	let mut owners = tasks.iter().map(|task| task.owner.as_str()).collect::<Vec<_>>();
	owners.sort();
	owners.dedup();
	owners
		.into_iter()
		.map(|owner| format!("- {owner}"))
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn render_deployment_preview(plan: &DeploymentPlan, tasks: &[Task]) -> String {
	format!(
		"deploy {} to {} with {} after syncing {} tracked tasks",
		plan.artifact,
		plan.region,
		plan.target,
		tasks.len()
	)
}