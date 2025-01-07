use fvp_unpacker_core::prelude::*;

const SINGLE_ENTRY_BIN: &[u8] = include_bytes!("single-entry.bin");
const MULTIPLE_ENTRIES_BIN: &[u8] = include_bytes!("multiple-entries.bin");

#[test]
fn parse_single_entry_bin_archive() {
  let arc = FvpBin::parse(SINGLE_ENTRY_BIN).unwrap();
  let entries = arc.entries();

  assert_eq!(entries.len(), 1);

  let entry = &entries[0];
  assert_eq!(entry.filename(), "filename");
  assert_eq!(entry.data(), b"data");
}

#[test]
fn parse_multiple_entries_bin_archive() {
  let arc = FvpBin::parse(MULTIPLE_ENTRIES_BIN).unwrap();
  let entries = arc.entries();

  assert_eq!(entries.len(), 3);

  let entry1 = &entries[0];
  assert_eq!(entry1.filename(), "file1");
  assert_eq!(entry1.data(), b"The answer to life");

  let entry2 = &entries[1];
  assert_eq!(entry2.filename(), "file2");
  assert_eq!(entry2.data(), b"the universe");

  let entry3 = &entries[2];
  assert_eq!(entry3.filename(), "file3");
  assert_eq!(entry3.data(), b"and everything");
}

#[test]
fn write_single_entry_bin_archive() {
  let mut arc = FvpBin::default();

  let entry = FvpBinEntry::new("filename", *b"data");
  arc.add_entry(entry);

  let mut bytes = Vec::new();
  arc.write(&mut bytes).unwrap();

  assert_eq!(bytes, SINGLE_ENTRY_BIN);
}

#[test]
fn write_multiple_entries_bin_archive() {
  let mut arc = FvpBin::default();

  arc.add_entry(FvpBinEntry::new("file1", *b"The answer to life"));
  arc.add_entry(FvpBinEntry::new("file2", *b"the universe"));
  arc.add_entry(FvpBinEntry::new("file3", *b"and everything"));

  let mut bytes = Vec::new();
  arc.write(&mut bytes).unwrap();

  assert_eq!(bytes, MULTIPLE_ENTRIES_BIN);
}
