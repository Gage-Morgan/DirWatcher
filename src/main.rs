use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // create a channel to recieve the events
    let (tx, rx) = channel();

    // Create a watcher object, delivering events via the channel
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Define the directory to watch
    let dir_to_watch = Path::new(&args[1]);

    // Start watching the firectory recursivley
    watcher.watch(dir_to_watch, RecursiveMode::Recursive)?;

    println!("Watching directory: {:?}", dir_to_watch);

    loop {
        match rx.recv() {
            Ok(event) => handle_event(event?),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn handle_event(event: Event) {
    for path in event.paths {
        match event.kind {
            notify::event::EventKind::Create(_) => {
                println!("File created: {:?}", path);
            }

            notify::event::EventKind::Modify(_) => {
                println!("File modified: {:?}", path);
            }

            notify::event::EventKind::Remove(_) => {
                println!("File deleted: {:?}", path);
            }

            _ => continue,
        }
    }
}
