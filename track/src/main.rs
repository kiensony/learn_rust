use std::{
    env, fs, io,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const USAGE: &str = "cargo track list [words ...] [--kind KIND] [--module MODULE] [--stage STAGE]\ncargo track show|next|check|test <id>\ncargo track hint <id> <1..3>";

struct Task<'a> {
    id: &'a str,
    position: &'a str,
    stage: &'a str,
    module: &'a str,
    phase: &'a str,
    kind: &'a str,
    title: &'a str,
    prerequisite: &'a str,
    tags: &'a str,
    diagnostics: &'a str,
    toolchain: &'a str,
    outcome: &'a str,
    requirements: &'a str,
}

impl<'a> Task<'a> {
    fn parse(line: &'a str) -> Result<Self, String> {
        let columns: Vec<_> = line.split('\t').collect();
        match columns.as_slice() {
            &[
                id,
                position,
                stage,
                module,
                phase,
                kind,
                title,
                prerequisite,
                tags,
                diagnostics,
                toolchain,
                outcome,
                requirements,
            ] => Ok(Self {
                id,
                position,
                stage,
                module,
                phase,
                kind,
                title,
                prerequisite,
                tags,
                diagnostics,
                toolchain,
                outcome,
                requirements,
            }),
            _ => Err(format!("invalid course catalog record {line:?}")),
        }
    }

    fn summary(&self) -> String {
        format!(
            "{}  {}  {}  [{} · {}]",
            self.position, self.id, self.title, self.module, self.kind
        )
    }

    fn legacy(&self) -> bool {
        self.id[5..].parse::<usize>().is_ok_and(|n| n <= 10)
    }

    fn direct(&self, action: &str) -> String {
        if self.legacy() {
            format!("cargo {action} -p {}", self.id.replace('-', "_"))
        } else if self.toolchain.starts_with("nightly") && action == "test" {
            format!(
                "cargo +{} miri test --manifest-path tasks/{}/Cargo.toml",
                self.toolchain, self.id
            )
        } else {
            format!(
                "cargo {action} --manifest-path tasks/{}/Cargo.toml",
                self.id
            )
        }
    }
}

fn list(out: &mut impl Write, tasks: &[Task], args: &[String]) -> Result<(), String> {
    let mut words = Vec::new();
    let mut filters = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--kind" | "--stage" | "--module" => {
                let value = iter
                    .next()
                    .filter(|v| !v.starts_with('-'))
                    .ok_or("missing filter value")?;
                filters.push((arg, value.to_lowercase()));
            }
            flag if flag.starts_with('-') => return Err(format!("unknown filter {flag}")),
            word => words.push(word.to_lowercase()),
        }
    }
    for task in tasks {
        let matches = filters.iter().all(|(name, value)| match name.as_str() {
            "--kind" => task.kind.eq_ignore_ascii_case(value),
            "--module" => task.module.to_lowercase().contains(value),
            _ => task.stage.to_lowercase().contains(value),
        });
        let fields = [
            task.id,
            task.title,
            task.module,
            task.stage,
            task.tags,
            task.diagnostics,
            task.toolchain,
            task.requirements,
            task.kind,
            task.phase,
        ]
        .join(" ")
        .to_lowercase();
        if matches && words.iter().all(|w| fields.contains(w)) {
            if let Err(error) = writeln!(out, "{}", task.summary()) {
                if error.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(error.to_string());
            }
        }
    }
    Ok(())
}

fn run(
    root: &Path,
    out: &mut impl Write,
    args: &[String],
) -> Result<u8, Box<dyn std::error::Error>> {
    let (action, rest) = args.split_first().ok_or(USAGE)?;
    if matches!(action.as_str(), "--help" | "-h" | "help") {
        writeln!(out, "{USAGE}")?;
        return Ok(0);
    }
    let catalog = fs::read_to_string(root.join("course/catalog.tsv"))
        .map_err(|e| format!("read catalog: {e}"))?;
    let tasks: Vec<_> = catalog
        .lines()
        .skip(1)
        .map(Task::parse)
        .collect::<Result<_, _>>()?;
    if action == "list" {
        list(out, &tasks, rest)?;
        if let [query] = rest {
            let code = query.to_ascii_uppercase();
            if code.len() == 5
                && code.starts_with('E')
                && code[1..].bytes().all(|b| b.is_ascii_digit())
            {
                let index = fs::read_to_string(root.join("course/error-index.tsv"))
                    .map_err(|e| format!("read compiler error index: {e}"))?;
                for row in index.lines().skip(1) {
                    let fields: Vec<_> = row.split('\t').collect();
                    if let [key, coverage, exercise, _, _, reason, _] = fields.as_slice() {
                        if *key == code {
                            writeln!(out, "{code}  {coverage}  {exercise}  {reason}")?;
                            if *coverage == "clinic" {
                                writeln!(
                                    out,
                                    "python3 tools/error_clinic.py {code} --prepare example.rs"
                                )?;
                            }
                            break;
                        }
                    }
                }
            }
        }
        return Ok(0);
    }
    let (raw, level) = match (action.as_str(), rest) {
        ("show" | "next" | "check" | "test", [id]) => (id, None),
        ("hint", [id, level]) => (
            id,
            Some(match level.as_str() {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                _ => return Err("hint level must be 1, 2 or 3".into()),
            }),
        ),
        _ => return Err(USAGE.into()),
    };
    let raw = raw.strip_prefix("task-").unwrap_or(raw);
    if raw.is_empty() || !raw.bytes().all(|c| c.is_ascii_digit()) {
        return Err("use a numeric task ID such as task-011".into());
    }
    let n: u32 = raw.parse().map_err(|_| "invalid task ID")?;
    let id = format!("task-{n:03}");
    let index = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or_else(|| format!("unknown {id}"))?;
    let task = &tasks[index];
    let dir = root.join("tasks").join(&id);
    match action.as_str() {
        "next" => {
            let message = tasks
                .get(index + 1)
                .map(Task::summary)
                .unwrap_or_else(|| "End of course.".into());
            writeln!(out, "{message}")?;
        }
        "hint" => {
            let path = if task.legacy() {
                root.join("course/legacy").join(format!("{id}.md"))
            } else {
                dir.join("HINTS.md")
            };
            let hints = fs::read_to_string(path)?;
            let hint = hints
                .lines()
                .filter(|s| !s.trim().is_empty())
                .nth(level.unwrap())
                .ok_or("missing hint")?;
            writeln!(out, "{hint}")?;
        }
        "show" => {
            let spec = fs::read_to_string(dir.join("PROBLEM.md"))?;
            writeln!(
                out,
                "{}\n{} · {} · prerequisite {}\nToolchain {} · {}\nStarter {}\n{}\n{}\n\n{spec}",
                task.summary(),
                task.stage,
                task.phase,
                task.prerequisite,
                task.toolchain,
                task.requirements,
                task.outcome,
                task.direct("check"),
                task.direct("test")
            )?;
        }
        "check" | "test" => {
            let mut cmd = Command::new("cargo");
            if task.toolchain.starts_with("nightly") && action == "test" {
                cmd.arg(format!("+{}", task.toolchain)).arg("miri");
            }
            cmd.arg(action);
            if task.legacy() {
                cmd.current_dir(root).arg("-p").arg(id.replace('-', "_"));
            } else {
                cmd.current_dir(dir);
            }
            return cmd
                .status()
                .map(|status| status.code().and_then(|c| c.try_into().ok()).unwrap_or(1))
                .map_err(|e| format!("start Cargo: {e}").into());
        }
        _ => unreachable!(),
    }
    Ok(0)
}

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    match run(
        &root,
        &mut io::stdout().lock(),
        &env::args().skip(1).collect::<Vec<_>>(),
    ) {
        Ok(code) => ExitCode::from(code),
        Err(e)
            if e.downcast_ref::<io::Error>()
                .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe) =>
        {
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("track: {e}");
            ExitCode::FAILURE
        }
    }
}
