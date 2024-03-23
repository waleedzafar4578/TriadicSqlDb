use crate::BaseControl;
use storge::column::Column;
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
        col_name: Vec<&str>,
        col_type: Vec<AttributeType>,
    ) -> bool {
         match self.db_select {
            true => {
                let mut tb: Table = Table::new(t_name);

                if col_name.len() == col_type.len() {
                    for (i, j) in col_name.iter().zip(col_type.iter()) {
                        tb.add_column(Column::new(i, j));
                    }
                    self.all_table.push(tb);
                    return true;
                }
                true
            }
            false => {
                println!("\n\n\nError: Please first select database!");
                false
            }
        }
    }
    pub fn drop_table(&mut self, t_name: &str) -> bool {
        match self.db_select {
            true => {
                for (i, cl) in self.all_table.clone().into_iter().enumerate() {
                    if cl.table_name() == t_name {
                        self.all_table.remove(i);
                        println!("Yes! This table is {} removed!", t_name);
                    }
                    
                }

                true
            }
            false => {
                println!("\n\n\nError: Please first select database!");
                false
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
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == t_name {
                _tb.add_col_data(c_name, c_data, d_status);
            }
        }
        false
    }
}
