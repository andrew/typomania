use std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
};

use proptest::prelude::*;

use crate::AuthorSet;

use super::{Check, Corpus, Package};

/// Upper bound on generated package name length, in bytes.
const MAX_NAME_BYTES: usize = 128;

#[derive(Debug, Clone, Default)]
pub struct TestPackage {
    pub authors: HashSet<String>,
    pub description: Option<String>,
}

impl TestPackage {
    pub fn new(author: &str) -> Self {
        Self {
            authors: [String::from(author)].into_iter().collect(),
            description: None,
        }
    }
}

impl AuthorSet for TestPackage {
    fn contains(&self, author: &str) -> bool {
        self.authors.contains(author)
    }
}

impl Package for TestPackage {
    fn authors(&self) -> &dyn AuthorSet {
        self
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn shared_authors(&self, other: &dyn AuthorSet) -> bool {
        self.authors.iter().any(|author| other.contains(author))
    }
}

struct NameTracker {
    known: HashMap<String, TestPackage>,
    seen: RwLock<HashMap<String, TestPackage>>,
}

impl NameTracker {
    fn new(known: &str) -> Self {
        Self {
            known: [String::from(known)]
                .into_iter()
                .map(|name| {
                    let package = TestPackage::new(&name);
                    (name, package)
                })
                .collect(),
            seen: RwLock::new(HashMap::default()),
        }
    }

    #[track_caller]
    fn assert_contains_exactly(&self, want: &[&str]) {
        let mut set = HashSet::new();
        for term in want {
            set.insert(String::from(*term));
        }

        let seen: HashSet<String> = self.seen.read().unwrap().keys().cloned().collect();

        assert_eq!(
            seen.symmetric_difference(&set)
                .cloned()
                .collect::<Vec<String>>(),
            Vec::<String>::new(),
        );
    }
}

impl Corpus for NameTracker {
    fn contains_name(&self, name: &str) -> crate::Result<bool> {
        Ok(if self.known.contains_key(name) {
            true
        } else {
            self.seen
                .write()
                .unwrap()
                .entry(name.into())
                .or_insert_with(|| TestPackage::new(name));
            false
        })
    }

    fn get(&self, name: &str) -> crate::Result<Option<&dyn Package>> {
        Ok(if let Some(package) = self.known.get(name) {
            Some(package)
        } else {
            // By using the package name as the author, no two packages will ever match.
            self.seen
                .write()
                .unwrap()
                .entry(name.into())
                .or_insert_with(|| TestPackage::new(name));

            None
        })
    }
}

#[track_caller]
pub(super) fn assert_check<C>(check: C, input: &str, want: &[&str]) -> crate::Result<()>
where
    C: Check,
{
    let names = NameTracker::new(input);

    check.check(&names, input, &TestPackage::new(input))?;
    names.assert_contains_exactly(want);

    Ok(())
}

/// Generates arbitrary package names of up to [`MAX_NAME_BYTES`] bytes, never splitting a
/// multi-byte character.
pub(super) fn name_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(any::<char>(), 0..=MAX_NAME_BYTES).prop_map(|chars| {
        let mut name = String::with_capacity(MAX_NAME_BYTES);
        for c in chars {
            if name.len() + c.len_utf8() > MAX_NAME_BYTES {
                break;
            }
            name.push(c);
        }
        name
    })
}

/// Asserts that running `check` against `name` neither panics nor returns an error.
#[track_caller]
pub(super) fn assert_no_panic<C>(check: C, name: &str)
where
    C: Check,
{
    let names = NameTracker::new(name);
    check.check(&names, name, &TestPackage::new(name)).unwrap();
}
