use jj_sample_repo::{config::AppConfig, output, tasks};

fn main() {
	let config = AppConfig::load();
	let tasks = tasks::sample_tasks();

	println!("{}", output::render_header(&config));
	println!();
	println!("{}", output::render_tasks(&tasks));
}