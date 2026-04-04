#[derive(Debug, Clone)]
pub struct Task {
	pub id: u32,
	pub title: String,
	pub status: String,
	pub owner: String,
	pub due_date: String,
}

pub fn sample_tasks() -> Vec<Task> {
	vec![
		Task {
			id: 1,
			title: "Ship onboarding polish".to_string(),
			status: "in-progress".to_string(),
			owner: "alex".to_string(),
			due_date: "2026-04-10".to_string(),
		},
		Task {
			id: 2,
			title: "Prepare release notes".to_string(),
			status: "todo".to_string(),
			owner: "sam".to_string(),
			due_date: "2026-04-14".to_string(),
		},
		Task {
			id: 3,
			title: "Audit background jobs".to_string(),
			status: "blocked".to_string(),
			owner: "jo".to_string(),
			due_date: "2026-04-20".to_string(),
		},
	]
}