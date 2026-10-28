// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `Exec` splitting and field-code expansion.

use std::path::Path;

/// Splits `Exec` and expands field codes that need no file or URL.
pub(super) fn expand_exec(
    exec: &str,
    name: &str,
    icon: Option<&str>,
    path: &Path,
) -> Result<Vec<String>, String> {
    let mut out = Vec::new();

    for arg in split_exec(exec) {
        expand_arg(&arg, name, icon, path, &mut out)?;
    }
    Ok(out)
}

/// Shell-like split on spaces; keeps quoted strings.
pub(super) fn split_exec(exec: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut chars = exec.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(next) = chars.next() {
                    cur.push(next);
                }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !cur.is_empty() {
                    args.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        args.push(cur);
    }
    args
}

/// Expands one argv token into `out`. File and URL codes drop the arg.
fn expand_arg(
    arg: &str,
    name: &str,
    icon: Option<&str>,
    path: &Path,
    out: &mut Vec<String>,
) -> Result<(), String> {
    if is_file_code(arg) {
        return Ok(());
    }
    if arg == "%i" {
        if let Some(icon) = icon {
            out.push("--icon".into());
            out.push(icon.into());
        }
        return Ok(());
    }

    let mut text = String::with_capacity(arg.len());
    let mut chars = arg.chars();

    while let Some(c) = chars.next() {
        if c != '%' {
            text.push(c);
            continue;
        }
        match chars.next() {
            Some('%') | None => text.push('%'),
            Some('c') => text.push_str(name),
            Some('k') => text.push_str(&path.to_string_lossy()),
            Some('i') => {
                return Err("%i must be its own Exec argument".into());
            }
            Some(code) if "fFuUdDnNvm".contains(code) => return Ok(()),
            Some(other) => {
                return Err(format!("unsupported Exec field code %{other}"));
            }
        }
    }
    out.push(text);
    Ok(())
}

fn is_file_code(arg: &str) -> bool {
    matches!(
        arg,
        "%f" | "%F" | "%u" | "%U" | "%d" | "%D" | "%n" | "%N" | "%v" | "%m"
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn splits_quotes_and_escapes() {
        assert_eq!(
            split_exec(r#"foo "bar baz" 'qux' a\ b"#),
            ["foo", "bar baz", "qux", "a b"]
        );
    }

    #[test]
    fn expands_field_codes() {
        let path = Path::new("/usr/share/applications/app.desktop");
        let argv = expand_exec(
            "app --name %c %i %% %k %f",
            "Demo",
            Some("demo"),
            path,
        )
        .unwrap();

        assert_eq!(
            argv,
            [
                "app",
                "--name",
                "Demo",
                "--icon",
                "demo",
                "%",
                path.to_str().unwrap(),
            ]
        );
    }

    #[test]
    fn drops_standalone_file_codes() {
        let argv = expand_exec("app %F", "Demo", None, Path::new("/x.desktop"))
            .unwrap();

        assert_eq!(argv, ["app"]);
    }
}
