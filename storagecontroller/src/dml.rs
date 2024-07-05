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
                    //_tb.add_recheck_data();
                }
            }
        }
        true
    }
    pub fn rename_table_name(&mut self, old_name: &String, new_name:&String) ->bool{
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == new_name.as_str()  {
                return false
            }
        }
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == old_name.as_str()  {
                _tb.set_table_name(new_name);
                return true
            }
        }
        false
    }
}
