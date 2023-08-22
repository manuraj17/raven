use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::process::Command;
use std::io::{self, Write};

#[derive(Deserialize, Clone, Debug)]
struct Sync {
    script: String
}
#[derive(Deserialize, Clone, Debug)]
struct Target {
    path: String,
    sync: Sync
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

fn parse_raven_config() -> RavenConfig {
    // Load config file into string
    let mut file =
        File::open("sample.config.toml").expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Parse config
    // let raven_config: RavenConfig = toml::from_str(&contents).unwrap();
    toml::from_str(&contents).unwrap()
}

fn display_changes(message: RavenMessage) {
    log::info!("File changed: {}", message.path);
    log::info!("Change type: {:?}", message.event.kind)
}

fn run_sync_command(target: Target) {
    let script = target.sync.script;
    let c = Command::new(script)
        // .arg("status")
        .current_dir("/Users/manu/Code/github/manuraj17/raven")
        .output()
        .expect("ls command failed to execute");

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
                let rm = RavenMessage { path: path.to_str().unwrap().to_owned(), event: event};
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

    let raven_config = parse_raven_config();

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
            event => display_changes(event)
        }
    }
}


