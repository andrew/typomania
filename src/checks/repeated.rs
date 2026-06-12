use itertools::Itertools;

use crate::Corpus;

use super::{Check, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by repeating one character.
pub struct Repeated;

impl Check for Repeated {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        let mut buf = String::new();
        for ((i, a), (_, b)) in name.char_indices().tuple_windows() {
            if a == b && a.is_ascii() {
                let after = name.get(i + 2..).unwrap_or_default();
                buf.clear();
                buf.reserve(i + 1 + after.len());
                buf.push_str(&name[..i]);
                buf.push(a);
                buf.push_str(after);

                if corpus.possible_squat(&buf, name, package)? {
                    squats.push(Squat::RepeatedCharacter(buf.clone()));
                }
            }
        }

        Ok(squats)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::checks::testutil::{assert_check, assert_no_panic, name_strategy};

    use super::*;

    proptest! {
        #[test]
        fn never_panics(name in name_strategy()) {
            assert_no_panic(Repeated, &name);
        }
    }

    #[test]
    fn test_repeated() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(Repeated, input, want)
        }

        test("", &[])?;
        test("a", &[])?;
        test("aa", &["a"])?;
        test("abc", &[])?;
        test("abbc", &["abc"])?;
        test("abbbc", &["abbc"])?;
        test("abbbbc", &["abbbc"])?;
        test("aaaaaa", &["aaaaa"])?;
        test("ۊۊ", &[])?;
        test("ۊaaۊ", &["ۊaۊ"])?;

        Ok(())
    }
}
