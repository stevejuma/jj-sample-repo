use jj_sample_repo::{
	config::{AppConfig, Command},
	output, tasks,
};

fn main() {
	let config = AppConfig::load();
	if let Err(error) = config.validate() {
		eprintln!("configuration error: {error}");
		std::process::exit(2);
	}

	let mut tasks = tasks::sample_tasks();
	if let Some(limit) = config.limit {
		tasks.truncate(limit);
	}

	println!("{}", output::render_header(&config));
	println!("{}", output::render_summary(&tasks));
	println!();

	match config.command {
		Command::List => println!("{}", output::render_tasks(&tasks)),
		Command::Owners => println!("{}", output::render_owners(&tasks)),
	}
}