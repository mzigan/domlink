use std::fmt;

pub(super) fn write_escaped<W: fmt::Write>(out: &mut W, s: &str) -> fmt::Result {
    let bytes = s.as_bytes();
    let mut last = 0;

    for (i, &b) in bytes.iter().enumerate() {
        let escaped = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };

        if last < i {
            out.write_str(&s[last..i])?;
        }

        out.write_str(escaped)?;
        last = i + 1;
    }

    if last < s.len() {
        out.write_str(&s[last..])?;
    }

    Ok(())
}

// for String, write_str never returns Err, this is guaranteed by the standard library
// so we use unwrap here
pub(super) fn escape_into_string(out: &mut String, s: &str) {
    write_escaped(out, s).unwrap()
}
