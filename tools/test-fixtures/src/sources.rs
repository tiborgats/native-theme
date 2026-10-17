//! Readers of a crate's own source text, for the tests that check what its
//! sources say: the lines outside a file's inline test module, and whether a
//! doc line names the document each of its `§` citations is in. The egui
//! connector's tripwires and the egui widgets crate's tests read their
//! sources through these, so the two read by one rule.

/// The lines of `text`, numbered from 1, before its inline `#[cfg(test)]`
/// module, which sits at a file's end: a `#[cfg(test)]` line, any attributes
/// after it, then a `mod name {` line. A `#[cfg(test)]` on anything else
/// (`mod tests;` in `lib.rs`, a test-only function) or inside a comment does
/// not end the scan.
#[must_use]
pub fn rendered_lines(text: &str) -> Vec<(usize, &str)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut rest = lines.as_slice();
    for number in 1.. {
        let Some((&line, after)) = rest.split_first() else {
            break;
        };
        if line.starts_with("#[cfg(test)]") && opens_inline_module(after) {
            break;
        }
        out.push((number, line));
        rest = after;
    }
    out
}

/// Whether the item after a `#[cfg(test)]` is an inline `mod name {`, past
/// its attributes, single-line or spread over lines up to one ending in `)]`.
#[must_use]
pub fn opens_inline_module(after: &[&str]) -> bool {
    let mut in_attribute = false;
    for line in after {
        let line = line.trim();
        if in_attribute {
            in_attribute = !line.ends_with(")]");
        } else if line.starts_with("#[") {
            in_attribute = !line.ends_with(']');
        } else {
            return line.starts_with("mod ") && line.ends_with('{');
        }
    }
    false
}

/// Whether every `§` on `doc` names its document: one of `names` stands
/// right before it, past spaces and backticks (`spec §4`,
/// `` `docs/platform-facts.md` §2.29 ``), or it continues a list or a range
/// whose `§` before it is named (`spec §4.2, §4.3`, `spec §5.1–§5.6`). A
/// name that starts with a letter or a digit stands as a word, so `spec`
/// does not name the `§` of `` `ResolvedFontSpec` §3 ``; one that starts
/// otherwise (`.md`) ends whatever it closes. `spec §1 and §2` leaves the
/// second unnamed.
#[must_use]
pub fn every_section_is_named(doc: &str, names: &[&str]) -> bool {
    doc.match_indices('§').all(|(at, _)| {
        let Some(before) = doc.get(..at) else {
            return false;
        };
        let text = before.trim_end_matches([' ', '`']);
        names.iter().any(|name| ends_with_name(text, name))
            || [", ", "; ", "–"].iter().any(|separator| {
                before.strip_suffix(separator).is_some_and(|list| {
                    list.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.')
                        .ends_with('§')
                })
            })
    })
}

/// Whether `text` ends with `name`, as a whole word where `name` starts with
/// a letter or a digit.
fn ends_with_name(text: &str, name: &str) -> bool {
    let Some(head) = text.strip_suffix(name) else {
        return false;
    };
    !name.starts_with(char::is_alphanumeric)
        || !head.ends_with(|c: char| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAMES: &[&str] = &["spec", "Spec", ".md", "CSS Fonts", "CSS Fonts Level 4"];

    /// A name, or a list or a range that continues a named `§`, names it; an
    /// `and`, another document's possessive, a name inside a longer word or
    /// nothing before it does not.
    #[test]
    fn a_section_is_named_by_the_name_or_the_list_before_it() {
        for named in [
            "/// spec §4",
            "//! Spec §4.2, §4.3 and spec §6",
            "/// (spec §6.18; §4.3)",
            "/// spec §5.1–§5.6",
            "/// `docs/platform-facts.md` §2.29",
            "/// platform-facts.md §2",
            "/// CSS Fonts Level 4 §5.1, then CSS Fonts §5.2",
            "/// the `spec` §7",
            "/// no section",
        ] {
            assert!(every_section_is_named(named, NAMES), "{named}");
        }
        for bare in [
            "/// §4",
            "/// spec §1 and §2",
            "/// spec §4; Breeze's §3",
            "/// §4.2, §4.3",
            "/// `ResolvedFontSpec` §3",
            "/// inspec §3",
            "/// spec_ §3",
        ] {
            assert!(!every_section_is_named(bare, NAMES), "{bare}");
        }
    }

    /// The names are the caller's: a line named by one list is bare under
    /// a list without that name.
    #[test]
    fn the_names_are_the_callers() {
        let line = "/// CSS Fonts §5.2";
        assert!(every_section_is_named(line, NAMES));
        assert!(!every_section_is_named(line, &["spec", "Spec", ".md"]));
    }

    /// A `#[cfg(test)]` named in a comment, on a test-only function or on
    /// `mod tests;` leaves the lines after it in; the inline test module,
    /// its attributes before it, is left out, and the lines keep their
    /// numbers.
    #[test]
    fn only_the_inline_test_module_is_left_out() {
        let source = "//! Its helpers sit under `#[cfg(test)]`.\n\
                      #[cfg(test)]\n\
                      mod tests;\n\
                      #[cfg(test)]\n\
                      pub(crate) fn stored() {}\n\
                      fn kept() {}\n\
                      #[cfg(test)]\n\
                      #[allow(\n\
                      clippy::unwrap_used,\n\
                      )]\n\
                      mod tests {\n\
                      fn left_out() {}\n\
                      }";
        let lines = rendered_lines(source);
        assert_eq!(lines.len(), 6, "{lines:#?}");
        assert_eq!(lines.last(), Some(&(6, "fn kept() {}")));
        assert_eq!(
            lines.first(),
            Some(&(1, "//! Its helpers sit under `#[cfg(test)]`."))
        );
    }

    /// A single-line attribute after the `#[cfg(test)]` is passed over too,
    /// and a file with no inline test module is read whole.
    #[test]
    fn an_attribute_before_the_module_is_passed_over() {
        assert!(opens_inline_module(&["#[allow(dead_code)]", "mod tests {"]));
        assert!(!opens_inline_module(&["mod tests;"]));
        assert!(!opens_inline_module(&["fn helper() {}"]));
        assert!(!opens_inline_module(&[]));
        assert_eq!(rendered_lines("a\nb").len(), 2);
    }
}
