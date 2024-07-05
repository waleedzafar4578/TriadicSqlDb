use triadic_logic::degree::Degree;

#[derive(Debug, Default, Clone)]
pub struct EqualOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}

impl EqualOperator {
    pub fn set(col_name:String,value:Option<String>,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct NotEqualOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}
impl NotEqualOperator {
    pub fn set(col_name:String, value: Option<String>, deg:Option<Degree>) ->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct GreaterEqualOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}
impl GreaterEqualOperator {
    pub fn set(col_name:String, value: Option<String>, deg:Option<Degree>) ->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct LessEqualOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}
impl LessEqualOperator {
    pub fn set(col_name:String, value: Option<String>, deg:Option<Degree>) ->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct GreaterOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}
impl GreaterOperator {
    pub fn set(col_name:String, value: Option<String>, deg:Option<Degree>) ->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct LessOperator {
    pub column_name: String,
    pub column_value: Option<String>,
    pub degree: Option<Degree>,
}
impl LessOperator {
    pub fn set(col_name:String, value: Option<String>, deg:Option<Degree>) ->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default,Clone)]
pub struct WhereClause {
    pub equal_operator: Option<EqualOperator>,
    pub not_equal_operator: Option<NotEqualOperator>,
    pub greater_equal_operator: Option<GreaterEqualOperator>,
    pub less_equal_operator: Option<LessEqualOperator>,
    pub greater_operator: Option<GreaterOperator>,
    pub less_operator: Option<LessOperator>,
}
#[derive(Debug, Default, Clone)]
pub struct SelectEntry {
    pub name: String,
    pub column_name: Vec<String>,
    pub where_clause: Option<WhereClause>,
}
