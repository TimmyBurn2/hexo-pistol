//! `--name value` command lines, in one spelling.
//!
//! Two curation binaries take a handful of flags each and refuse everything
//! else by name (CLAUDE.md rule 3). The grammar and its refusals live here for
//! the reason [`crate::count`] gives for numbers: a rule with two
//! implementations is a rule that can hold two answers, and "unknown flag" is
//! exactly the kind of message that drifts.
//!
//! The usage text travels as an argument rather than living here, because it is
//! the one part that is genuinely each tool's own.

/// The `--name value` pairs of a command line, in the order given.
///
/// A bare word, a flag with nothing after it, and a flag whose value is another
/// flag are three different mistakes and say so. The last one is worth its own
/// message: `--corpus --out-dir x` is a forgotten value, and silently taking
/// `--out-dir` as the corpus path would run the tool on a file named after a
/// flag.
pub fn pairs<'a>(words: &[&'a str], usage: &str) -> Result<Vec<(&'a str, &'a str)>, String> {
    let mut found = Vec::new();
    let mut rest = words;
    while let [name, tail @ ..] = rest {
        if !name.starts_with("--") {
            return Err(format!("expected a flag, got `{name}`\n\n{usage}"));
        }
        let [value, tail @ ..] = tail else {
            return Err(format!("`{name}` needs a value"));
        };
        if value.starts_with("--") {
            return Err(format!("`{name}` needs a value, got the flag `{value}`"));
        }
        found.push((*name, *value));
        rest = tail;
    }
    Ok(found)
}

/// The one value of a flag that must appear exactly once.
pub fn one<'a>(found: &[(&'a str, &'a str)], name: &str, usage: &str) -> Result<&'a str, String> {
    let mut matches = found.iter().filter(|(flag, _)| *flag == name);
    let Some((_, value)) = matches.next() else {
        return Err(format!("`{name}` is required\n\n{usage}"));
    };
    if matches.next().is_some() {
        return Err(format!("`{name}` is given more than once"));
    }
    Ok(value)
}

/// Refuse a flag this command does not have.
pub fn only(found: &[(&str, &str)], allowed: &[&str], usage: &str) -> Result<(), String> {
    for (flag, _) in found {
        if !allowed.contains(flag) {
            return Err(format!(
                "unknown flag `{flag}`; this command takes {}\n\n{usage}",
                allowed.join(", ")
            ));
        }
    }
    Ok(())
}
