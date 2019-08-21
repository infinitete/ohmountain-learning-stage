/// 创建一个类型为i32的Vector
pub fn new() -> Vec<i32> {
    vec![]
}

/// 判断超出作用域后赋值仍然有效
pub fn out_scope() -> Vec<i32> {
    let v = vec![];

    {
        // 作用域内的v在离开作用域后将被抛弃
        let v = vec![1, 2, 3, 4];
    }

    v
}

/// 使用Vector::get方法安全地获取Vector的值
pub fn safe_read(n: i32, i: usize) -> Option<i32> {
    let mut v = vec![];

    // [0,1,2,3....,n]
    for i in 0..n + 1 {
        v.push(i)
    }

    match v.get(i) {
        Some(value) => Some(*value),
        None => None,
    }
}

/// 不可变遍历
pub fn iterator(n: i32) {
    let mut v = vec![];

    // [0,1,2,3....,n]
    for i in 0..n + 1 {
        v.push(i)
    }

    let mut index = 0;

    for i in &v {
        assert_eq!(i, v.get(index).unwrap_or(&-1));
        index += 1;
    }
}

/// 可变遍历Vector
pub fn iterator_mut<F>(v: &mut Vec<i32>, reducer: F) where F:Fn(&mut i32) -> i32
{
    for i in v {
        *i = reducer(i);
    }
}

pub enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String),
}

/// 使用枚举来储存多种类型
pub fn enum_items() -> Vec<SpreadsheetCell> {
    vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_new() {
        let new_vec = super::new();
        assert_eq!(new_vec.len(), 0);
    }

    #[test]
    fn test_fn_out_scope() {
        let v = super::out_scope();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_fn_safe_read() {
        match super::safe_read(10, 20) {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }

        match super::safe_read(10, 10) {
            Some(v) => assert_eq!(v, 10),
            None => assert_eq!(true, true),
        }
    }

    #[test]
    fn test_fn_iterator() {
        super::iterator(10);
    }

    #[test]
    fn test_fn_iterator_mut() {
        let mut v = vec![1,2,3,4];
        super::iterator_mut(&mut v, |x| (*x) * (*x));

        for x in 1..v.len() + 1 {
            assert_eq!(x*x, v[x - 1] as usize);
        }
    }

    #[test]
    fn test_fn_enum_itetms() {
        for v in super::enum_items() {
            match v {
                super::SpreadsheetCell::Int(value) => assert_eq!(value, 3),
                super::SpreadsheetCell::Float(value) => assert_eq!(value, 10.12),
                super::SpreadsheetCell::Text(value) => assert_eq!(value, "blue".to_owned()),
            }
        }
    }
}
