pub trait Table {
    fn create_table_with_data(table_name: &'static str, data: Vec<Self>) -> String
    where
        Self: Sized,
    {
        format!(
            "{}{}",
            Self::create_table(table_name),
            Self::insert_data(table_name, data)
        )
    }
    fn create_table(table_name: &'static str) -> String {
        let mut columns = Self::columns()
            .iter()
            .map(|(name, r#type)| format!("{name} {type}"))
            .collect::<Vec<String>>();
        if let Some(foreign_key_data) = Self::foreign_keys() {
            columns.extend(foreign_key_data.iter().map(|datum| {
                format!(
                    "FOREIGN KEY ({}) REFERENCES {} ({})",
                    datum.0, datum.1, datum.2
                )
            }))
        }
        format!("CREATE TABLE {table_name} ({});\n", columns.join(", "))
    }
    fn insert_data(table_name: &'static str, data: Vec<Self>) -> String
    where
        Self: Sized,
    {
        format!(
            "INSERT INTO {table_name} ({}) VALUES \n\t{};\n",
            Self::columns()
                .iter()
                .map(|(name, _)| name.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            data.iter()
                .map(|datum| format!("('{}')", datum.row().join("', '")))
                .collect::<Vec<String>>()
                .join(",\n\t")
        )
    }
    fn columns() -> Vec<(&'static str, &'static str)>;
    fn row(&self) -> Vec<String>;
    fn foreign_keys() -> Option<Vec<(&'static str, &'static str, &'static str)>> {
        None
    }
}
