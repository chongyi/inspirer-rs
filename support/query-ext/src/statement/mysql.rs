use crate::statement::IntoStatement;
use crate::sort::{SortStatement, Sort};
use sqlx::mysql::MySql;

impl<T: AsRef<str>> IntoStatement<MySql> for SortStatement<T> {
    fn statement(&self) -> String {
        self.iter()
            .map(|option| match option {
                Sort::Asc(field) => format!("{} asc", field.as_ref()),
                Sort::Desc(field) => format!("{} desc", field.as_ref()),
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    fn full_statement(&self) -> String {
        let statement = self.statement();
        if statement.is_empty() {
            statement
        } else {
            format!("order by {}", statement)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{SortStatement, Sort};
    use crate::test::SortColumn;

    #[test]
    fn test_statement() {
        let mut statement = SortStatement::<SortColumn>::default();
        statement.push(Sort::Desc(SortColumn::Id));
        statement.push(Sort::Asc(SortColumn::CreateTime));

        assert_eq!(
            "content.id desc,content.create_time asc",
            statement.statement()
        );
        assert_eq!(
            "order by content.id desc,content.create_time asc",
            statement.full_statement()
        );
    }
}