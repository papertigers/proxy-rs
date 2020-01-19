use chrono::prelude::*;

const DEFAULT_UNIT: f64 = 1024_f64;
const SUFFIX: &[&str] = &["k", "M", "G", "T", "P", "E"];

/// Takes the number of bytes and converts it to a human readable string
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
