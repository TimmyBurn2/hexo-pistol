use std::str::FromStr;

/// A count, or why the text is not one.
///
/// Refused: an empty string, a sign, leading zeros on anything but `0` itself,
/// spaces, and a value the target type cannot hold. The explanation is the caller's
/// to wrap with whatever the count was for.
pub fn plain_count<T: FromStr>(text: &str) -> Result<T, String> {
    if text.is_empty() {
        return Err(String::from(
            "a plain non-negative decimal, and this is empty",
        ));
    }
    if let Some(sign) = text.chars().next().filter(|c| *c == '+' || *c == '-') {
        return Err(format!(
            "a plain non-negative decimal, written without a leading `{sign}`"
        ));
    }
    if !text.chars().all(|c| c.is_ascii_digit()) {
        return Err(String::from("a plain non-negative decimal, digits only"));
    }
    if text.len() > 1 && text.starts_with('0') {
        return Err(String::from(
            "a plain non-negative decimal, written without leading zeros: one number, \
             one spelling",
        ));
    }
    text.parse::<T>()
        .map_err(|_| String::from("a number this build can hold"))
}
