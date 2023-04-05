use std::{
    fs::File,
    io::{self, BufRead},
    process::{Command, Stdio},
};

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let programs = load_programs();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select program")
        .default(0)
        .items(&programs)
        .interact()
        .unwrap();
    let program = &programs[selection];

    let command_parts: Vec<&str> = program.command.split(" ").collect();

    let mut cmd = Command::new(command_parts[0]);
    let process = cmd.args(command_parts.iter().skip(1));

    process
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect(format!("failed to start {}", program.name).as_str());

    std::process::exit(0);
}

fn load_programs() -> Vec<Program> {
    glob::glob("/usr/share/applications/*.desktop")
        .unwrap()
        .filter_map(|result| result.ok())
        .map(|path| {
            let mut name = None;
            let mut command = None;
            let file = File::open(path).unwrap();
            for line in io::BufReader::new(file).lines() {
                let line = line.unwrap();
                if line.starts_with("Name=") {
                    name = Some(line.trim_start_matches("Name=").to_string());
                } else if line.starts_with("Exec=") {
                    command = Some(line.trim_start_matches("Exec=").to_string());
                }
            }
            if let (Some(name), Some(command)) = (name, command) {
                Ok(Program { name, command })
            } else {
                Err(())
            }
        })
        .filter_map(|result| result.ok())
        .collect()
}

struct Program {
    name: String,
    command: String,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
