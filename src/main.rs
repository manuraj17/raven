use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Deserialize, Clone, Debug)]
struct Sync {
    script: String,
}
#[derive(Deserialize, Clone, Debug)]
struct Target {
    path: String,
    sync: Sync,
}

#[derive(Deserialize, Clone)]
struct RavenConfig {
    targets: Vec<Target>,
}

#[derive(Debug, Clone)]
struct RavenMessage {
    path: String,
    event: Event,
}

fn config_from_config_folder() -> RavenConfig {
    let home_dir = std::env::var_os("HOME").expect("no home directory");
    let mut config_path = std::path::PathBuf::new();
    config_path.push(home_dir);
    config_path.push(".config/raven/config.toml");

    let config_content = if let Ok(content) = std::fs::read(&config_path) {
        String::from_utf8_lossy(&content).to_string()
    } else {
        "This is the default content\n".to_owned()
    };

    toml::from_str(&config_content).unwrap()
}

fn config_from_passed_path(path: &str) -> RavenConfig {
    let mut config_path = std::path::PathBuf::new();
    config_path.push(path);

    let config_content = if let Ok(content) = std::fs::read(&config_path) {
        String::from_utf8_lossy(&content).to_string()
    } else {
        "This is the default content\n".to_owned()
    };

    toml::from_str(&config_content).unwrap()
}

fn parse_raven_config(file_path: Option<&str>) -> RavenConfig {
    match file_path {
        None => config_from_config_folder(),
        Some(val) => config_from_passed_path(val),
    }
}

fn display_changes(message: RavenMessage) {
    log::info!("File changed: {}", message.path);
    log::info!("Change type: {:?}", message.event.kind)
}

fn run_sync_command(target: Target) {
    let script = target.sync.script;
    let target_path = std::path::PathBuf::from(target.path);

    let cdir = if target_path.is_dir() {
        target_path.to_str()
    } else {
        target_path.parent().unwrap().to_str()
    };

    let c = Command::new(script)
        .current_dir(cdir.unwrap())
        .output()
        .expect("sync command failed to execute");

    if c.status.success() {
        log::info!("Sync succeeded");
        log::info!("Sync output: ");
        io::stdout().write_all(&c.stdout).unwrap();
    }
}

fn watch(target: Target, ms: std::sync::mpsc::Sender<RavenMessage>) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    let path = Path::new(&target.path);
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                let rm = RavenMessage {
                    path: path.to_str().unwrap().to_owned(),
                    event: event,
                };
                println!("Publishing Change: {rm:?}");
                if let Err(error) = ms.send(rm) {
                    log::error!("Error: {error:?}");
                }
                println!("Staring sync");
                run_sync_command(target.clone());
            }
            Err(error) => {
                log::error!("Error: {error:?}");
            }
        }
    }

    Ok(())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let raven_config = parse_raven_config(None);

    // Channel to receive updates
    let (master_sender, master_receiver): (Sender<RavenMessage>, Receiver<RavenMessage>) =
        channel();

    // loop through each target and print the value
    for target in raven_config.targets.iter() {
        let sender_clone = master_sender.clone();
        let c_target = target.clone();

        log::info!("Watching {}", c_target.path);

        thread::spawn(|| {
            if let Err(error) = watch(c_target, sender_clone) {
                log::error!("Error: {error:?}");
            }
        });
    }

    for res in master_receiver {
        match res {
            event => display_changes(event),
        }
    }
}

// TODO: Update tests
#[cfg(test)]
mod tests {
    use crate::parse_raven_config;

    #[test]
    fn test_parse_config() {
        let test_config_file_path = "./sample.config.toml";
        let parsed_config = parse_raven_config(Some(&test_config_file_path));

        assert_eq!(parsed_config.targets.iter().len(), 1);
        assert_eq!(
            parsed_config.targets.first().unwrap().path,
            "/Users/user/sample.txt"
        );
        assert_eq!(
            parsed_config.targets.first().unwrap().sync.script,
            "./sync.sh"
        );
    }
}
