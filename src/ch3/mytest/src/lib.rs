#[derive(Debug, PartialEq)]
// Debug: フォーマッタを利用して値を出力できるようにする。 PartialEq: 構造体の各要素を比較できるようにする

struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // 外側の要素を利用
    #[test]
    fn item_test(){
        let apple1 = GItem{
            name: String::from("apple"),
            price: 2400,
        };
        let mut apple2 = GItem{
            name: "apple".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        assert_eq!(apple1, apple2);
    }
}
