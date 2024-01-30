use core::ops::Range;
use csv::StringRecord;

pub(super) fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let mut res = String::new();
    for r in char_pos {
        res.push_str(
            &(line
                .chars()
                .enumerate()
                .filter(|(i, _)| i >= &r.start)
                .take_while(|(i, _)| i < &r.end)
                .map(|(_, c)| c)
                .collect::<String>()),
        );
    }

    res
}

pub(super) fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let mut res = vec![];
    for r in byte_pos {
        res.extend_from_slice(
            &line
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(i, _)| i >= &r.start)
                .take_while(|(i, _)| i < &r.end)
                .map(|(_, c)| *c)
                .collect::<Vec<u8>>(),
        );
    }

    String::from_utf8_lossy(&res).to_string()
}

pub(super) fn extract_fields<'a>(
    line: &'a StringRecord,
    field_pos: &[Range<usize>],
) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| line.get(i)))
        .collect()
}
#[cfg(test)]
mod unit_tests {
    use super::{extract_bytes, extract_chars, extract_fields};
    use csv::StringRecord;
    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 1..2, 4..5]), "áb".to_string());
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(extract_fields(&rec, &[0..1, 2..3]), &["Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }
}
