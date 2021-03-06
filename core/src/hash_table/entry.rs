use hash;
use multihash::Hash;
use serde_json;
use std::hash::{Hash as StdHash, Hasher};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    content: String,

    // @TODO do NOT serialize entry_type in Entry as it should only be in Header
    // @see https://github.com/holochain/holochain-rust/issues/80
    entry_type: String,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        // @TODO is this right?
        // e.g. two entries with the same content but different type are equal
        // @see https://github.com/holochain/holochain-rust/issues/85
        self.hash() == other.hash()
    }
}

/// implement Hash for Entry to match PartialEq logic
// @TODO is this right?
// e.g. two entries with the same content but different type are equal
// @see https://github.com/holochain/holochain-rust/issues/85
impl StdHash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.hash(state);
    }
}

impl Entry {
    /// build a new Entry from passed content
    /// an Entry is immutable, this is important for absolutely everything downstream
    /// an entry is not valid until paired with a header and included in a chain.
    /// @see chain::header::Header
    /// @see chain::pair::Pair
    pub fn new(entry_type: &str, content: &str) -> Entry {
        Entry {
            entry_type: entry_type.to_string(),
            content: content.to_string(),
        }
    }

    /// hashes the entry
    pub fn hash(&self) -> String {
        // @TODO - this is the wrong string being hashed
        // @see https://github.com/holochain/holochain-rust/issues/103
        let string_to_hash = &self.content;

        // @TODO the hashing algo should not be hardcoded
        // @see https://github.com/holochain/holochain-rust/issues/104
        hash::str_to_b58_hash(string_to_hash, Hash::SHA2256)
    }

    /// content getter
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// entry_type getter
    pub fn entry_type(&self) -> String {
        self.entry_type.clone()
    }

    /// returns true iff the entry is valid
    pub fn validate(&self) -> bool {
        // always valid if immutable and new() enforces validity
        true
    }

    /// returns the key used for lookups in chain, HT, etc.
    /// note that entry keys have a parallel API to header/pair keys, e.g. chain.get_entry()
    pub fn key(&self) -> String {
        self.hash()
    }

    /// serialize the Entry to a canonical JSON string
    ///
    /// @TODO return canonical JSON
    /// @see https://github.com/holochain/holochain-rust/issues/75
    pub fn to_json(&self) -> String {
        // @TODO error handling
        // @see https://github.com/holochain/holochain-rust/issues/168
        serde_json::to_string(&self).expect("should serialize without error")
    }

    /// deserialize an Entry from a canonical JSON string
    ///
    /// # Panics
    ///
    /// Panics if the string passed isn't valid JSON.
    /// @TODO accept canonical JSON
    /// @see https://github.com/holochain/holochain-rust/issues/75
    /// @TODO don't return invalid entries
    pub fn from_json(s: &str) -> Entry {
        let entry: Entry = serde_json::from_str(s).expect("JSON should be valid");
        entry
    }
}

#[cfg(test)]
pub mod tests {
    use super::Entry;

    /// dummy entry type
    pub fn test_type() -> String {
        "testEntryType".into()
    }

    /// dummy entry type, same as test_type()
    pub fn test_type_a() -> String {
        test_type()
    }

    /// dummy entry type, differs from test_type()
    pub fn test_type_b() -> String {
        "testEntryTypeB".into()
    }

    /// dummy entry content
    pub fn test_content() -> String {
        "test entry content".into()
    }

    /// dummy entry content, same as test_content()
    pub fn test_content_a() -> String {
        test_content()
    }

    /// dummy entry content, differs from test_content()
    pub fn test_content_b() -> String {
        "other test entry content".into()
    }

    /// dummy entry
    pub fn test_entry() -> Entry {
        Entry::new(&test_type(), &test_content())
    }

    /// the correct hash for test_entry()
    pub fn test_entry_hash() -> String {
        "QmbXSE38SN3SuJDmHKSSw5qWWegvU7oTxrLDRavWjyxMrT".into()
    }

    /// dummy entry, same as test_entry()
    pub fn test_entry_a() -> Entry {
        test_entry()
    }

    /// dummy entry, differs from test_entry()
    pub fn test_entry_b() -> Entry {
        Entry::new(&test_type_b(), &test_content_b())
    }

    #[test]
    /// tests for PartialEq
    fn eq() {
        let c1 = "foo";
        let c2 = "bar";
        let t1 = "a";
        let t2 = "b";

        // same type and content is equal
        assert_eq!(Entry::new(t1, c1), Entry::new(t1, c1));

        // same type different content is not equal
        assert_ne!(Entry::new(t1, c1), Entry::new(t1, c2));

        // same content different type is equal
        // @see https://github.com/holochain/holochain-rust/issues/85
        assert_eq!(Entry::new(t1, c1), Entry::new(t2, c1));

        // different content different type is not equal
        assert_ne!(Entry::new(t1, c1), Entry::new(t2, c2));
    }

    #[test]
    /// tests that hash equality matches PartialEq
    fn eq_hash() {
        let c1 = "foo";
        let c2 = "bar";
        let t1 = "a";
        let t2 = "b";

        // same type and content is equal
        assert_eq!(Entry::new(t1, c1).hash(), Entry::new(t1, c1).hash());

        // same type different content is not equal
        assert_ne!(Entry::new(t1, c1).hash(), Entry::new(t1, c2).hash());

        // same content different type is equal
        // @see https://github.com/holochain/holochain-rust/issues/85
        assert_eq!(Entry::new(t1, c1).hash(), Entry::new(t2, c1).hash());

        // different content different type is not equal
        assert_ne!(Entry::new(t1, c1).hash(), Entry::new(t2, c2).hash());
    }

    #[test]
    /// tests for Entry::new()
    fn new() {
        let c = "foo";
        let t = "bar";
        let e = Entry::new(t, c);

        assert_eq!(e.content(), c);
        assert_ne!(e.hash(), "");
        assert!(e.validate());
    }

    #[test]
    /// test entry.hash() against a known value
    fn hash_known() {
        assert_eq!(test_entry_hash(), test_entry().hash());
    }

    #[test]
    /// test that the content changes the hash
    fn hash_content() {
        let t = "bar";
        let c1 = "baz";
        let c2 = "foo";

        let e1 = Entry::new(t, c1);
        let e2 = Entry::new(t, c1);
        let e3 = Entry::new(t, c2);

        // same content same hash
        assert_eq!(e1.hash(), e2.hash());

        // different content, different hash
        assert_ne!(e1.hash(), e3.hash());
    }

    #[test]
    /// test that the entry type does NOT change the hash
    fn hash_entry_type() {
        let t1 = "barType";
        let t2 = "fooo";
        let c = "barr";

        let e1 = Entry::new(t1, c);
        let e2 = Entry::new(t2, c);

        assert_eq!(e1.hash(), e2.hash());
    }

    #[test]
    /// tests for entry.content()
    fn content() {
        let c = "baz";
        let t = "foo";
        let e = Entry::new(t, c);

        assert_eq!("baz", e.content());
    }

    #[test]
    /// tests for entry.entry_type()
    fn entry_type() {
        let t = "bar";
        let c = "foo";
        let e = Entry::new(t, c);

        assert_eq!(t, e.entry_type());
    }

    #[test]
    /// tests for entry.validate()
    fn validate() {
        let t = "";
        let c = "";
        let e = Entry::new(t, c);

        assert!(e.validate());
    }

    #[test]
    /// tests for entry.key()
    fn key() {
        assert_eq!(test_entry().hash(), test_entry().key());
    }

    #[test]
    /// test that we can round trip through JSON
    fn json_round_trip() {
        let e = test_entry_a();
        let expected = r#"{"content":"test entry content","entry_type":"testEntryType"}"#;
        assert_eq!(expected, e.to_json());
        assert_eq!(e, Entry::from_json(expected));
        assert_eq!(e, Entry::from_json(&e.to_json()));
    }
}
