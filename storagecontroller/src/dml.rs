use crate::BaseControl;
use storge::column::{Column, Constraints};
use storge::table::Table;
use triadic_logic::datatype::AttributeType;
use triadic_logic::degree::Degree;

impl BaseControl {
    pub fn get_table(&self, name: String) -> Option<&Table> {
        self.all_table.iter().find(|&tb| name == *tb.get_table_name())
    }
    pub fn get_column(&self, table_name: String, column_name: String) -> Option<&Column> {
        if let Some(tb) = self.all_table.first() {
           if table_name == *tb.get_table_name(){
                return Some(tb.get_full_column(column_name).unwrap());
            }
                
        }
        None
    }
    pub fn add_table(
        &mut self,
        t_name: &str,
        col_name: Vec<String>,
        col_type: Vec<(AttributeType,Constraints)>,
    ) -> String {
        //println!("Yes! Come in engine side.");
        return match self.db_select {
            true => {
                if self.search_table(t_name) {
                    return format!("{} this table is already in database!", t_name);
                }
                let mut tb: Table = Table::new(t_name);

                if col_name.len() == col_type.len() {
                    for (i, (j, c)) in col_name.iter().zip(col_type.iter()) {
                        tb.add_column(Column::new(i, j, c.clone()));
                    }
                    self.all_table.push(tb);
                    return format!("{} table is created in database!", t_name);
                }
                format!("Something wrong with this {}  \
                or his column or datatype", t_name)
            }
            false => {
                "Error: Please first select database!".to_string()
            }
        }
    }
    pub fn drop_table(&mut self, t_name: &str) -> String {
        return match self.db_select {
            true => {
                for (i, cl) in self.all_table.clone().into_iter().enumerate() {
                    if cl.table_name() == t_name {
                        self.all_table.remove(i);
                        return format!("Yes! This table is {} removed!", t_name);
                    }
                }
                "Table is not found in database!".to_string()
            }
            false => {
                "Error: Please first select database!".to_string()
            }
        }
    }
    pub fn insert_to_table(
        &mut self,
        t_name: &str,
        c_name: &str,
        c_data: &str,
        d_status: Degree,
    ) -> bool {
        println!("{}:{}:{}:{}",t_name,c_name,c_data,d_status);
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == t_name  {
                if !_tb.add_col_data(c_name, c_data, d_status) {
                    return false
                }
                else {
                    _tb.add_recheck_data();
                }
            }
        }
        true
    }
}
