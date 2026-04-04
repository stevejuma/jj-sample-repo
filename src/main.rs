use jj_sample_repo::{config::AppConfig, output, tasks};

fn main() {
	let config = AppConfig::load();
	let tasks = tasks::sample_tasks();

	println!("{}", output::render_header(&config));
	println!("{}", output::render_summary(&tasks));
	println!();
	println!("{}", output::render_tasks(&tasks));
}