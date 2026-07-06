use std::{
    env,
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use anyhow::{Context, Result, bail};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

const DEFAULT_WATCH_PATHS: &[&str] = &[
    "src",
    "assets",
    "locales",
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
    ".cargo",
];

fn main() {
    if let Err(err) = run() {
        eprintln!("[dev-reload] {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::parse(env::args().skip(1).collect())?;
    if config.show_help {
        print!("{}", Config::help());
        return Ok(());
    }

    let root = env::current_dir().context("resolve current directory")?;
    let mut runner = DevReload::new(root, config);
    runner.run()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    release: bool,
    debounce_ms: u64,
    watch_paths: Vec<PathBuf>,
    app_args: Vec<OsString>,
    show_help: bool,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Self> {
        let mut release = false;
        let mut debounce_ms = 400_u64;
        let mut watch_paths = Vec::new();
        let mut app_args = Vec::new();
        let mut show_help = false;

        let mut i = 0;
        let mut passthrough = false;
        while i < args.len() {
            let arg = &args[i];
            if passthrough {
                app_args.push(OsString::from(arg));
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--" => {
                    passthrough = true;
                    i += 1;
                }
                "--release" => {
                    release = true;
                    i += 1;
                }
                "--debounce-ms" => {
                    let value = args
                        .get(i + 1)
                        .context("missing value for --debounce-ms")?;
                    debounce_ms = value
                        .parse::<u64>()
                        .with_context(|| format!("invalid --debounce-ms value: {value}"))?;
                    i += 2;
                }
                "--watch" => {
                    let value = args.get(i + 1).context("missing value for --watch")?;
                    watch_paths.push(PathBuf::from(value));
                    i += 2;
                }
                "--help" | "-h" => {
                    show_help = true;
                    i += 1;
                }
                other => {
                    bail!("unknown argument: {other}\n\n{}", Self::help());
                }
            }
        }

        if watch_paths.is_empty() {
            watch_paths = DEFAULT_WATCH_PATHS.iter().map(PathBuf::from).collect();
        }

        Ok(Self {
            release,
            debounce_ms,
            watch_paths,
            app_args,
            show_help,
        })
    }

    fn help() -> &'static str {
        "\
Usage:
  cargo run --example dev_reload -- [options] [-- <ashell-args>]
  cargo dev-reload [options] [-- <ashell-args>]

Options:
  --release             Build and run target/release/ashell
  --debounce-ms <ms>    Debounce file events before rebuild (default: 400)
  --watch <path>        Additional or replacement watch path; may be repeated
  -h, --help            Show this help

Notes:
  - This is restart-based development reload, not state-preserving hot reload.
  - On file change, the running app is stopped first, then rebuilt and restarted.
  - Default watch set: src, assets, locales, Cargo.toml, Cargo.lock, build.rs, .cargo
"
    }
}

struct DevReload {
    root: PathBuf,
    config: Config,
    child: Option<Child>,
}

impl DevReload {
    fn new(root: PathBuf, config: Config) -> Self {
        Self {
            root,
            config,
            child: None,
        }
    }

    fn run(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = self.build_watcher(tx)?;

        for watch_path in self.resolved_watch_paths() {
            let mode = if watch_path.is_dir() {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            watcher
                .watch(&watch_path, mode)
                .with_context(|| format!("watch {}", watch_path.display()))?;
            eprintln!("[dev-reload] watching {}", watch_path.display());
        }

        self.rebuild_and_restart("initial start")?;

        loop {
            let events = collect_change_batch(&rx, Duration::from_millis(self.config.debounce_ms))?;
            let summary = summarize_events(&events);
            self.rebuild_and_restart(&summary)?;
        }
    }

    fn build_watcher(
        &self,
        tx: mpsc::Sender<notify::Result<Event>>,
    ) -> notify::Result<RecommendedWatcher> {
        notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
    }

    fn resolved_watch_paths(&self) -> Vec<PathBuf> {
        self.config
            .watch_paths
            .iter()
            .map(|path| self.root.join(path))
            .filter(|path| path.exists())
            .collect()
    }

    fn rebuild_and_restart(&mut self, reason: &str) -> Result<()> {
        eprintln!("[dev-reload] trigger: {reason}");
        self.stop_child()?;
        self.build_app()?;
        self.start_app()?;
        Ok(())
    }

    fn build_app(&self) -> Result<()> {
        let mut command = Command::new("cargo");
        command.current_dir(&self.root).arg("build").arg("--bin").arg("ashell");
        if self.config.release {
            command.arg("--release");
        }
        command.stdin(Stdio::null());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        let status = command.status().context("spawn cargo build")?;
        ensure_success(status, "cargo build")
    }

    fn start_app(&mut self) -> Result<()> {
        let executable = self.binary_path();
        let mut command = Command::new(&executable);
        command.current_dir(&self.root);
        command.args(&self.config.app_args);
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        let child = command
            .spawn()
            .with_context(|| format!("start {}", executable.display()))?;
        eprintln!("[dev-reload] started {}", executable.display());
        self.child = Some(child);
        Ok(())
    }

    fn stop_child(&mut self) -> Result<()> {
        let Some(mut child) = self.child.take() else {
            return Ok(());
        };

        if child.try_wait().context("poll child process")?.is_some() {
            return Ok(());
        }

        // Stop first, then rebuild, to avoid executable replacement issues on Windows.
        child.kill().context("stop running app")?;
        let _ = child.wait();
        eprintln!("[dev-reload] stopped running app");
        Ok(())
    }

    fn binary_path(&self) -> PathBuf {
        let mut base = match env::var_os("CARGO_TARGET_DIR") {
            Some(path) => PathBuf::from(path),
            None => self.root.join("target"),
        };
        if base.is_relative() {
            base = self.root.join(base);
        }
        let profile = if self.config.release { "release" } else { "debug" };
        base.join(profile).join(format!("ashell{}", env::consts::EXE_SUFFIX))
    }
}

fn collect_change_batch(
    rx: &Receiver<notify::Result<Event>>,
    debounce: Duration,
) -> Result<Vec<Event>> {
    let first = rx.recv().context("watch channel closed")??;
    let mut events = vec![first];
    loop {
        match rx.recv_timeout(debounce) {
            Ok(Ok(event)) => events.push(event),
            Ok(Err(err)) => eprintln!("[dev-reload] watcher error: {err}"),
            Err(mpsc::RecvTimeoutError::Timeout) => return Ok(events),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "watch channel closed").into())
            }
        }
    }
}

fn summarize_events(events: &[Event]) -> String {
    let mut labels = Vec::new();
    for path in events
        .iter()
        .flat_map(|event| event.paths.iter())
        .filter_map(|path| relative_label(path))
    {
        if !labels.contains(&path) {
            labels.push(path);
        }
        if labels.len() == 3 {
            break;
        }
    }

    if labels.is_empty() {
        format!("{} filesystem events", events.len())
    } else {
        format!("{} filesystem events ({})", events.len(), labels.join(", "))
    }
}

fn relative_label(path: &Path) -> Option<String> {
    let cwd = env::current_dir().ok()?;
    let relative = path.strip_prefix(cwd).unwrap_or(path);
    Some(relative.display().to_string())
}

fn ensure_success(status: ExitStatus, action: &str) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        bail!("{action} failed with status {status}");
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, DEFAULT_WATCH_PATHS};
    use std::path::PathBuf;

    #[test]
    fn parses_defaults() {
        let config = Config::parse(vec![]).expect("parse defaults");
        assert!(!config.release);
        assert_eq!(config.debounce_ms, 400);
        assert_eq!(
            config.watch_paths,
            DEFAULT_WATCH_PATHS
                .iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        );
        assert!(config.app_args.is_empty());
        assert!(!config.show_help);
    }

    #[test]
    fn parses_custom_options_and_passthrough_args() {
        let config = Config::parse(vec![
            "--release".into(),
            "--debounce-ms".into(),
            "900".into(),
            "--watch".into(),
            "src".into(),
            "--watch".into(),
            "README.md".into(),
            "--".into(),
            "--foo".into(),
            "bar".into(),
        ])
        .expect("parse custom args");

        assert!(config.release);
        assert_eq!(config.debounce_ms, 900);
        assert_eq!(config.watch_paths, vec![PathBuf::from("src"), PathBuf::from("README.md")]);
        assert_eq!(config.app_args, vec!["--foo", "bar"]);
    }

    #[test]
    fn rejects_unknown_args() {
        let err = Config::parse(vec!["--wat".into()]).expect_err("unknown arg should fail");
        let message = format!("{err:#}");
        assert!(message.contains("unknown argument"));
    }
}
