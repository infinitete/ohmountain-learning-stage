
/// 创建一个空的字符串
pub fn new_empty() -> String {
    String::new()
}

/// 使用一个字面量来创建字符串
pub fn new_with_value(value: &str) -> String {
    String::from(value)
}

/// 测试
#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_new_empty() {
        assert!(super::new_empty().is_empty())
    }

    #[test]
    fn test_fn_new_with_value() {
        let value = "hello，两只Tigger";
        assert_eq!(value.to_owned(), super::new_with_value(value));
    }

    #[test]
    fn test_update_string() {
        let mut s1 = super::new_with_value("两只");
        s1.push_str("老虎");
        assert_eq!("两只老虎".to_string(), s1);

        let mut s2 = super::new_empty();
        s2.push('X');
        assert_eq!("X".to_string(), s2);
        assert_ne!("x".to_string(), s2);
    }

    #[test]
    fn test_string_with_utf8() {
        let s1 = super::new_with_value("两只小脑斧 are tow tigers");
        assert_eq!(30, s1.len());
    }

    #[test]
    fn test_iter_string() {
        let s1: String = super::new_with_value("两只小脑斧 are tow tigers");

        let mut size = 0;

        // 不要直接索引字符串
        for c in s1.chars() {
            assert!(s1.contains(c));
            size += 1;
        }

        assert_eq!(20, size);
    }
}
