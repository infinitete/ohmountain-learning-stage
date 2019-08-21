use std::collections::HashMap;

/// 新建一个空的HashMap
///
/// `HashMap`是同质的，所有键必须有相同的类型
pub fn new() -> HashMap<String, u8> {
    HashMap::new()
}

/// 把两个Vector压缩成一个HashMap
///
/// `iter`后做一次`map`，取出引用的值，否则会变成双重引用，即`&&str`和`&u8`
pub fn zip(names: Vec<&str>, scores: Vec<u8>) -> HashMap<&str, u8> {
    let map: HashMap<&str, u8> = names
        .iter()
        .map(|x| *x)
        .zip(scores.iter().map(|x| x.to_owned()))
        .collect();

    map
}

#[cfg(test)]
mod tests {

    use super::HashMap;

    #[test]
    pub fn test_fn_new() {
        let mut map: HashMap<String, u8> = super::new();
        assert!(map.is_empty());

        map.entry("真的".to_string()).or_insert(10u8);
        assert!(map.contains_key(&"真的".to_string()));
    }

    #[test]
    pub fn test_fn_zip() {
        let names = vec!["张三", "李四", "王五"];
        let scores = vec![10, 20, 30];

        // 变成 ["张三" => 10, "李四" => 20, "王五" => 30]
        let name_score = super::zip(names, scores);
        assert_eq!(false, name_score.is_empty());

        assert_eq!(name_score.get(&"张三").unwrap_or(&0), &10);
        assert_eq!(name_score.get(&"李四").unwrap_or(&0), &20);
        assert_eq!(name_score.get(&"王五").unwrap_or(&0), &30);
    }

    #[test]
    // 此处暂时无法处理panic
    #[should_panic(expected="borrow of moved value: `name`")]
    pub fn test_hashmap_ownship() {
        let mut map: HashMap<String, u8> = super::new();
        let name = String::from("张三");
        let score = 10u8;

        map.insert(name, score);

        // `name` 和 `score` 将不再可用
        // moved before
        //
        // println!("{} - {}", name, score);
        panic!("borrow of moved value: `name`");
    }
}
