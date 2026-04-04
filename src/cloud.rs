#[derive(Debug, Clone)]
pub struct DeploymentPlan {
	pub target: String,
	pub region: String,
	pub artifact: String,
}

pub fn deployment_plan(target: &str) -> DeploymentPlan {
	let region = match target {
		"edge" => "us-east-1",
		"batch" => "eu-west-1",
		_ => "local-dev",
	};

	DeploymentPlan {
		target: target.to_string(),
		region: region.to_string(),
		artifact: format!("task-tracker-{target}.tar.gz"),
	}
}