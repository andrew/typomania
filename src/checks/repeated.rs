use itertools::Itertools;

use crate::Corpus;

use super::{util, Check, Package, Squat};

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
        for (i, (a, b)) in name.chars().tuple_windows().enumerate() {
            if a == b && a.is_ascii() {
                util::rebuild_name_into(&mut buf, name, i, 2, &format!("{a}"));
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
    use crate::checks::testutil::assert_check;

    use super::*;

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

        Ok(())
    }
}
