pub(super) fn rebuild_name_into(
    buf: &mut String,
    orig: &str,
    index: usize,
    replace: usize,
    replacement: &str,
) {
    let after = orig.get(index + replace..).unwrap_or_default();
    buf.clear();
    buf.reserve(index + replacement.len() + after.len());
    buf.push_str(&orig[..index]);
    buf.push_str(replacement);
    buf.push_str(after);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rebuild_name(orig: &str, index: usize, replace: usize, replacement: &str) -> String {
        let mut buf = String::new();
        rebuild_name_into(&mut buf, orig, index, replace, replacement);
        buf
    }

    #[test]
    fn test_rebuild_name() {
        assert_eq!("foobar", rebuild_name("foobar", 3, 0, ""));
        assert_eq!("fooxbar", rebuild_name("foobar", 3, 0, "x"));
        assert_eq!("fooxar", rebuild_name("foobar", 3, 1, "x"));
        assert_eq!("fxbar", rebuild_name("foobar", 1, 2, "x"));
        assert_eq!("fxxbar", rebuild_name("foobar", 1, 2, "xx"));
    }
}
