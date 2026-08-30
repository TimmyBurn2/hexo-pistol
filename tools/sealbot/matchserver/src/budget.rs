/// One seat's budget, and with it the mode the seat must find.
///
/// A closed enum for the same reason `Budget` is one in the engine
/// (docs/decisions.md D-4): an absent budget is an error, never a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PistolBudget {
    /// `go nodes <n>`, under `mode = "instrument"`.
    Nodes(u64),
    /// `go movetime <ms>`, under `mode = "play"`.
    MovetimeMs(u64),
}

impl PistolBudget {
    /// The mode a seat with this budget must hand back on its handshake.
    pub fn required_mode(self) -> &'static str {
        match self {
            PistolBudget::Nodes(_) => "instrument",
            PistolBudget::MovetimeMs(_) => "play",
        }
    }

    /// The budget word the handshake's `id budgets` line must advertise.
    pub fn required_budget_word(self) -> &'static str {
        match self {
            PistolBudget::Nodes(_) => "nodes",
            PistolBudget::MovetimeMs(_) => "movetime",
        }
    }

    /// The `go` line this seat sends.
    pub fn go_line(self) -> String {
        match self {
            PistolBudget::Nodes(n) => format!("go nodes {n}"),
            PistolBudget::MovetimeMs(ms) => format!("go movetime {ms}"),
        }
    }
}
