use chrono::prelude::*;
use std::time::Instant;

const DEFAULT_UNIT: f64 = 1024_f64;
const SUFFIX: &[&str] = &["k", "M", "G", "T", "P", "E"];

/// Takes the number of bytes and converts it to a human readable string.
pub fn pretty_bytes(b: u64) -> String {
    let b = b as f64;

    if b < DEFAULT_UNIT {
        return format!("{:.0} B", b);
    }

    let idx = (b.log10() / DEFAULT_UNIT.log10()) as usize;
    let b = b / DEFAULT_UNIT.powi(idx as i32);
    let suffix = SUFFIX[idx.wrapping_sub(1)];

    format!("{:.1} {}B", b, suffix)
}

pub fn log<S: AsRef<str>>(message: S) {
    let dt: DateTime<Local> = Local::now();
    println!(
        "{} {}",
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        message.as_ref()
    );
}

/// Return the quotient and remainder between two u128s.
fn div_rem(lhs: u128, rhs: u128) -> (u128, u128) {
    (lhs / rhs, lhs % rhs)
}

// Note: I would rather use something like gethrtime(3c) instead of an `Instant`. However, it's
// probably fine since an `Instant` in rust is monotonically increasing and guaranteed to be no
// less than the previous measured value.

/// Takes a `Instant` and pretty prints the delta in days, hours, minutes, and seconds.
pub fn duration_delta(i: Instant) -> String {
    let delta = Instant::now().duration_since(i);
    let ms = delta.as_millis();
    let (days, ms) = div_rem(ms, 86_400_000);
    let (hours, ms) = div_rem(ms, 3_600_000);
    let (minutes, ms) = div_rem(ms, 60_000);
    let seconds = ms as f32 / 1000f32;
    if days > 0 {
        format!("{}d{}h{}m{:.1}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h{}m{:.1}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m{:.1}s", minutes, seconds)
    } else {
        format!("{:.1}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_bytes_test() {
        assert_eq!("1 B", pretty_bytes(1));
        assert_eq!("1.0 kB", pretty_bytes(1024));
        assert_eq!("1.0 MB", pretty_bytes(1024u64.pow(2)));
        assert_eq!("1.0 GB", pretty_bytes(1024u64.pow(3)));
        assert_eq!("1.0 TB", pretty_bytes(1024u64.pow(4)));
        assert_eq!("1.0 PB", pretty_bytes(1024u64.pow(5)));
        assert_eq!("1.0 EB", pretty_bytes(1024u64.pow(6)));
    }
}
