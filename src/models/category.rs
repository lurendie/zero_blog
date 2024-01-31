use rbatis::crud_table;

#[derive(Debug, Clone)]
#[crud_table]
pub struct Category{
    pub id:u16,
    pub name:String
}

// impl CRUDTable for Category{
//     fn table_name() -> String {
//         "category".to_string()
//     }
//
//     fn table_columns() -> String {
//         "id,category_name".to_string()
//     }
// }