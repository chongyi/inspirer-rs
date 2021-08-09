/// 排序选项
#[derive(Serialize, Deserialize, AsRefStr, Debug, Clone)]
#[serde(tag = "mode", content = "column")]
pub enum Sort<T> {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc(T),
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc(T),
}

/// 排序语句
pub type SortStatement<T> = Vec<Sort<T>>;

#[cfg(test)]
mod tests {
    use crate::sort::{SortStatement, Sort};
    use crate::test::SortColumn;

    #[test]
    fn test_as_ref() {
        assert_eq!("asc", Sort::Asc(()).as_ref());
        assert_eq!("desc", Sort::Desc(()).as_ref());
    }

    #[test]
    fn test_serialize() {
        let mut statement = SortStatement::<SortColumn>::default();
        statement.push(Sort::Desc(SortColumn::Id));
        statement.push(Sort::Asc(SortColumn::CreateTime));

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Options {
            sorts: SortStatement<SortColumn>,
        }

        assert_eq!(
            "sorts[0][mode]=desc&sorts[0][column]=id&sorts[1][mode]=asc&sorts[1][column]=create_time",
            serde_qs::to_string(&Options { sorts: statement}).unwrap()
        );

        let a:Sort<SortColumn> = serde_qs::from_str("mode=desc&column=id").unwrap();
        if let Sort::Desc(inner) = a {
            assert_eq!("content.id", inner.as_ref());
        } else {
            assert!(false);
        }

        let b = serde_qs::from_str::<Options>("sorts[0][mode]=desc&sorts[0][column]=id&sorts[1][mode]=asc&sorts[1][column]=create_time");
        assert!(b.is_ok());
        println!("{:?}", b);
    }
}