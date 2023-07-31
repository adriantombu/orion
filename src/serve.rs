use crate::build::run as run_build;
use crate::Config;
use anyhow::{Context, Result};
use console::style;
use notify::event::DataChange::Content;
use notify::event::{RemoveKind, RenameMode};
use notify::{Event, FsEventWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use rouille::Response;
use std::path::{Path, PathBuf};

/// Runs a local server to navigate the blog
pub fn run() -> Result<()> {
    run_build().context("Failed to build the blog")?;

    let mut watcher = get_watcher()?;
    watcher.watch(Path::new("posts"), RecursiveMode::Recursive)?;
    watcher.watch(Path::new("static"), RecursiveMode::Recursive)?;
    watcher.watch(Path::new("themes"), RecursiveMode::Recursive)?;

    println!("{}", style("Listening to localhost:1337...").green());

    let config = &Config::new().context("Failed to retrieve the configuration")?;
    let path = config.build_path.clone();

    rouille::start_server("localhost:1337", move |request| {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            return response;
        }

        Response::redirect_302("/index.html")
    });
}

fn get_watcher() -> Result<FsEventWatcher> {
    Ok(RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(create_kind)
                    if create_kind == notify::event::CreateKind::File
                        || create_kind == notify::event::CreateKind::Folder =>
                {
                    rebuild(&event.paths, "created");
                }

                notify::EventKind::Modify(kind)
                    if kind == notify::event::ModifyKind::Data(Content)
                        || kind == notify::event::ModifyKind::Name(RenameMode::To) =>
                {
                    rebuild(&event.paths, "modified");
                }

                notify::EventKind::Remove(RemoveKind::Any) => {
                    rebuild(&event.paths, "deleted");
                }
                _ => (),
            },
            Err(e) => println!("watch error: {e:?}"),
        },
        notify::Config::default(),
    )?)
}

fn rebuild(paths: &[PathBuf], action: &str) {
    if let Some(path) = paths.get(0) {
        println!(
            "{}",
            style(format!(
                "{} has been {action}. Reloading...",
                path.display()
            ))
            .yellow()
        );

        if let Err(e) = run_build() {
            println!("{}", style(format!("Error building the blog: {e:?}")).red());
        };
    }
}
