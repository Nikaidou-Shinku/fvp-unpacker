pub fn human_readable_size(size: usize) -> String {
  let mut size = size as f64;
  let mut scale = 0;

  while size >= 1024.0 {
    size /= 1024.0;
    scale += 1;
  }

  // FIXME: handle very large number
  const UNITS: [&str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

  format!("{:.2} {}", size, UNITS[scale])
}
