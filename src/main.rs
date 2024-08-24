use color_print::cprint;
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

    cprint!("Watching directory: <bright-blue>{:?}</>\n", dir_to_watch);

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
                cprint!("File created: <bright-green>{:?}</>\n", path);
            }

            notify::event::EventKind::Modify(_) => {
                cprint!("File modified: <bright-yellow>{:?}</>\n", path);
            }

            notify::event::EventKind::Remove(_) => {
                cprint!("File deleted: <bright-red>{:?}</>\n", path);
            }

            _ => continue,
        }
    }
}
