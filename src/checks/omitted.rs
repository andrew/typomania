use crate::Corpus;

use super::{util, Check, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by omitting one character.
pub struct Omitted {
    alphabet: Vec<String>,
}

impl Omitted {
    /// Instantiates an omitted character check.
    ///
    /// `alphabet` is the list of characters that are valid in a package name.
    pub fn new(alphabet: &str) -> Self {
        Self {
            alphabet: alphabet.chars().map(String::from).collect(),
        }
    }
}

impl Check for Omitted {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();
        let mut buf = String::new();

        for i in name
            .char_indices()
            .map(|(i, _)| i)
            .chain(std::iter::once(name.len()))
        {
            for c in self.alphabet.iter() {
                util::rebuild_name_into(&mut buf, name, i, 0, c);
                if corpus.possible_squat(&buf, name, package)? {
                    squats.push(Squat::OmittedCharacter(buf.clone()));
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
            assert_no_panic(Omitted::new("abc"), &name);
        }
    }

    #[test]
    fn test_omitted() -> crate::Result<()> {
        assert_check(
            Omitted::new("abc"),
            "xyz",
            &[
                "axyz", "bxyz", "cxyz", "xayz", "xbyz", "xcyz", "xyaz", "xybz", "xycz", "xyza",
                "xyzb", "xyzc",
            ],
        )?;
        assert_check(Omitted::new("a"), "-ۊ-", &["a-ۊ-", "-aۊ-", "-ۊa-", "-ۊ-a"])?;

        Ok(())
    }
}
