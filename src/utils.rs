use std::fmt::Display;

#[derive(Debug)]
pub enum FileSize {
    B,
    K,
    M,
    G
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FileSize::B => "B",
            FileSize::K => "KB",
            FileSize::M => "MB",
            FileSize::G => "GB",
        };
        write!(f, "{}", s)
    }
}


pub fn shorten_file_size(size: i64) -> (f64, FileSize) {
    let mut compressed: f64 = size  as f64;
    let mut order = 0;
    if compressed == 0.0 {
        return (compressed, FileSize::B);
    }
    while order < 4 && compressed > 1000.0 {
        compressed /= 1000.0;
        order += 1;
    }
    let file_size = match order {
        0 => FileSize::B,
        1 => FileSize::K,
        2 => FileSize::M,
        _ => FileSize::G
    };
    (compressed, file_size)
}