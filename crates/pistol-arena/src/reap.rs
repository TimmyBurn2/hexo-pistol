//! How a child dies, and how the arena finds out.
//!
//! Split from `channel.rs` on that line: that module owns the CONVERSATION, and
//! this one owns the process's end. The distinction the code here makes is the
//! one docs/decisions.md D-159 turns on, so it is worth a file of its own.
//!
//! A child that exited with a CODE chose to, and a deterministic answer to a
//! deterministic input may be adjudicated: it forfeits. A child killed from
//! outside — a signal, an OOM kill — is machine-dependence, and adjudicating it
//! would make the verdict a function of how loaded the box was. So is a child
//! that closes its pipe and then refuses to die, which is why the wait is
//! bounded: those abandon the run.

use std::process::{Child, ExitStatus};
use std::time::{Duration, Instant};

/// How often the reaper looks again at a child that has not exited yet.
///
/// Short enough that reaping a cooperative child costs nothing measurable, long
/// enough that waiting out an uncooperative one is not a spin loop.
pub const REAP_POLL_MS: u64 = 5;

/// How a child that closed its pipe died.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Death {
    /// It chose to exit, with this code. A deterministic answer: forfeits.
    Exited(i32),
    /// It was killed from outside, or its status could not be read. Abandons
    /// the run.
    Killed(String),
}

/// How a child that closed its pipe died, waiting no longer than the run's
/// own watchdog for it to actually go.
///
/// The bound is the point, and it was missing in the first implementation.
/// A closed pipe and an exited process are TWO EVENTS: the reader thread
/// ends at EOF on stdout, which a child can produce while continuing to
/// run. An unbounded `wait()` there blocks in a place the watchdog cannot
/// see — control has already left [`Channel::receive`] — so an engine that
/// closed its output and kept running hung the whole arena with no report
/// written at all, defeating both the liveness device and the promise that
/// an abandoned run keeps its evidence (docs/decisions.md D-159, D-160).
///
/// A child that outlives the watchdog after closing its pipe is killed and
/// classified as [`Death::Killed`], which routes it to abandon-the-run:
/// "it stopped answering and would not die" is machine-dependent in exactly
/// the way a timeout is, and must not adjudicate a game.
pub fn death(child: &mut Child, timeout_ms: u64) -> Death {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return classify(status),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Death::Killed(format!(
                        "it closed its pipe and was still running {timeout_ms} ms later, so \
                         it was killed; a child that stops answering and will not exit is \
                         not an answer this arena can adjudicate"
                    ));
                }
                std::thread::sleep(Duration::from_millis(REAP_POLL_MS));
            }
            Err(io) => {
                return Death::Killed(format!("its exit status could not be read: {io}"));
            }
        }
    }
}

/// The exit-status split this module's doc argues for.
fn classify(status: ExitStatus) -> Death {
    if let Some(code) = status.code() {
        return Death::Exited(code);
    }
    // Naming the signal is the difference between diagnosing an OOM kill
    // and diagnosing a crash, which is the whole point of separating this
    // from an ordinary exit (docs/decisions.md D-159).
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return Death::Killed(format!(
                "terminated by signal {signal} rather than exiting on its own"
            ));
        }
    }
    Death::Killed(String::from(
        "terminated by a signal rather than exiting on its own",
    ))
}
