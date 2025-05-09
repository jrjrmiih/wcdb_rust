use crate::base::cpp_object::{CppObject, CppObjectTrait};
use crate::orm::field::Field;
use crate::utils::ToCString;
use crate::winq::column::Column;
use crate::winq::common_table_expression::CommonTableExpression;
use crate::winq::conflict_action::ConflictAction;
use crate::winq::expression::Expression;
use crate::winq::expression_convertible::ExpressionConvertibleTrait;
use crate::winq::identifier::{CPPType, Identifier, IdentifierStaticTrait, IdentifierTrait};
use crate::winq::ordering_term::OrderingTerm;
use crate::winq::qualified_table::QualifiedTable;
use crate::winq::statement::{Statement, StatementTrait};
use core::ffi::c_size_t;
use std::ffi::{c_char, c_int, c_void, CString};
use std::fmt::Debug;
use std::os::raw::{c_double, c_long};
use std::ptr::{null, null_mut};

extern "C" {
    fn WCDBRustStatementUpdate_create() -> *mut c_void;

    fn WCDBRustStatementUpdate_configWith(
        cpp_obj: *mut c_void,
        expressions: *const *mut c_void,
        expressions_length: c_int,
    );

    fn WCDBRustStatementUpdate_configRecursive(cpp_obj: *mut c_void);

    fn WCDBRustStatementUpdate_configTable(
        cpp_obj: *mut c_void,
        type_i: c_int,
        table: *mut c_void,
        table_name: *const c_char,
    );

    fn WCDBRustStatementUpdate_configColumnsToBindParameters(
        cpp_obj: *mut c_void,
        columns_type: c_int,
        columns_void_vec: *const *mut c_void,
        columns_string_vec: *const *const c_char,
        columns_vec_len: c_int,
    );

    fn WCDBRustStatementUpdate_configCondition(cpp_obj: *mut c_void, condition: *mut c_void);

    fn WCDBRustStatementUpdate_configOrders(
        cpp_obj: *mut c_void,
        orders: *const *mut c_void,
        vec_len: c_size_t,
    );
    fn WCDBRustStatementUpdate_configLimitCount(
        cpp_obj: *mut c_void,
        config_type: c_int,
        limit: c_long,
    );
    fn WCDBRustStatementUpdate_configOffset(
        cpp_obj: *mut c_void,
        config_type: c_int,
        offset: c_long,
    );
    fn WCDBRustStatementUpdate_configConfliction(cpp_obj: *mut c_void, action: c_int);

    fn WCDBRustStatementUpdate_configValue(
        cpp_obj: *mut c_void,
        cpp_type: c_int,
        // todo denxudong 补充 *mut c_void
        // arg_cpp_obj: *mut c_void,
        int_value: *mut c_void,
        double_value: c_double,
        string_value: *const c_char,
    );

    fn WCDBRustStatementUpdate_configColumns(
        cpp_obj: *mut c_void,
        cpp_type: c_int,
        columns: *const *mut c_void,
        column_names: *const *const c_char,
        vec_len: c_int,
    );

    fn WCDBRustStatementUpdate_configLimitRange(
        cpp_obj: *mut c_void,
        from_type: c_int,
        from: c_long,
        to_type: c_int,
        to: c_long,
    );
}

#[derive(Debug)]
pub struct StatementUpdate {
    statement: Statement,
}

impl CppObjectTrait for StatementUpdate {
    fn set_cpp_obj(&mut self, cpp_obj: *mut c_void) {
        self.statement.set_cpp_obj(cpp_obj);
    }

    fn get_cpp_obj(&self) -> *mut c_void {
        self.statement.get_cpp_obj()
    }

    fn release_cpp_object(&mut self) {
        self.statement.release_cpp_object();
    }
}

impl IdentifierTrait for StatementUpdate {
    fn get_description(&self) -> String {
        self.statement.get_description()
    }
}

impl IdentifierStaticTrait for StatementUpdate {
    fn get_type() -> i32 {
        CPPType::UpdateSTMT as i32
    }
}

impl StatementTrait for StatementUpdate {
    fn is_write_statement(&self) -> bool {
        self.statement.is_write_statement()
    }
}

impl StatementUpdate {
    pub fn new() -> Self {
        let cpp_obj = unsafe { WCDBRustStatementUpdate_create() };
        StatementUpdate {
            statement: Statement::new_with_obj(cpp_obj),
        }
    }

    pub fn with(&self, expressions: &Vec<CommonTableExpression>) -> &Self {
        if expressions.is_empty() {
            return self;
        }
        let mut cpp_obj_vec: Vec<*mut c_void> = Vec::with_capacity(expressions.len());
        for x in expressions {
            cpp_obj_vec.push(CppObject::get(x));
        }
        unsafe {
            WCDBRustStatementUpdate_configWith(
                self.get_cpp_obj(),
                cpp_obj_vec.as_ptr(),
                cpp_obj_vec.len() as c_int,
            );
        }
        self
    }

    pub fn with_recursive(&self, expressions: &Vec<CommonTableExpression>) -> &Self {
        if expressions.is_empty() {
            return self;
        }
        let mut cpp_obj_vec: Vec<*mut c_void> = Vec::with_capacity(expressions.len());
        for x in expressions {
            cpp_obj_vec.push(CppObject::get(x));
        }
        unsafe {
            WCDBRustStatementUpdate_configWith(
                self.get_cpp_obj(),
                cpp_obj_vec.as_ptr(),
                cpp_obj_vec.len() as c_int,
            );
        }
        unsafe { WCDBRustStatementUpdate_configRecursive(self.get_cpp_obj()) }
        self
    }

    pub fn update(&self, table_name: &str) -> &Self {
        let c_table_name = CString::new(table_name).unwrap_or_default();
        unsafe {
            WCDBRustStatementUpdate_configTable(
                self.get_cpp_obj(),
                CPPType::String as i32,
                null_mut(),
                c_table_name.as_ptr(),
            );
        }
        self
    }

    pub fn update_qualified_table(&self, table: QualifiedTable) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configTable(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(&table),
                CppObject::get(&table),
                null(),
            )
        }
        self
    }

    pub fn or_replace(&self) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configConfliction(
                self.get_cpp_obj(),
                ConflictAction::Replace as i32,
            )
        }
        self
    }

    pub fn or_rollback(&self) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configConfliction(
                self.get_cpp_obj(),
                ConflictAction::Rollback as i32,
            )
        }
        self
    }

    pub fn or_abort(&self) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configConfliction(
                self.get_cpp_obj(),
                ConflictAction::Abort as i32,
            )
        }
        self
    }

    pub fn or_fail(&self) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configConfliction(
                self.get_cpp_obj(),
                ConflictAction::Fail as i32,
            )
        }
        self
    }

    pub fn or_ignore(&self) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configConfliction(
                self.get_cpp_obj(),
                ConflictAction::Ignore as i32,
            )
        }
        self
    }

    pub fn set_columns_to_bind_parameters<T>(&self, fields: &Vec<&Field<T>>) -> &Self {
        if fields.is_empty() {
            return self;
        }
        let columns_void_vec_len = fields.len() as i32;
        let mut c_void_vec: Vec<*mut c_void> = Vec::with_capacity(fields.len());
        for field in fields {
            c_void_vec.push(field.get_cpp_obj());
        }

        unsafe {
            WCDBRustStatementUpdate_configColumnsToBindParameters(
                self.get_cpp_obj(),
                CPPType::Column as i32,
                c_void_vec.as_ptr(),
                null(),
                columns_void_vec_len,
            );
        }
        self
    }

    pub fn set_column_objs_to_bind_parameters(&self, columns: &Vec<Column>) -> &Self {
        if columns.is_empty() {
            return self;
        }
        let columns_vec_len = columns.len();
        let mut c_void_vec: Vec<*mut c_void> = Vec::with_capacity(columns_vec_len);
        for column in columns {
            c_void_vec.push(column.get_cpp_obj());
        }
        unsafe {
            WCDBRustStatementUpdate_configColumnsToBindParameters(
                self.get_cpp_obj(),
                CPPType::Column as i32,
                c_void_vec.as_ptr(),
                null(),
                columns_vec_len as c_int,
            );
        }
        self
    }

    // todo dengxudong 重要不紧急
    // public StatementUpdate setColumnsToValues(@NotNull Column[] columns, @NotNull Object[] values)

    pub fn set_columns(&self, columns: &Vec<&Column>) -> &Self {
        if columns.is_empty() {
            return self;
        }
        let mut columns_void_vec: Vec<*mut c_void> = Vec::with_capacity(columns.len());
        for x in columns {
            columns_void_vec.push(CppObject::get(*x));
        }
        unsafe {
            WCDBRustStatementUpdate_configColumns(
                self.get_cpp_obj(),
                CPPType::Column as c_int,
                columns_void_vec.as_ptr(),
                null(),
                columns_void_vec.len() as c_int,
            )
        }
        self
    }

    pub fn set_column_names(&self, columns: &Vec<String>) -> &Self {
        if columns.is_empty() {
            return self;
        }

        let mut columns_void_vec: Vec<*const c_char> = Vec::with_capacity(columns.len());
        for x in columns {
            columns_void_vec.push(x.to_cstring().into_raw());
        }

        unsafe {
            WCDBRustStatementUpdate_configColumns(
                self.get_cpp_obj(),
                CPPType::String as c_int,
                null(),
                columns_void_vec.as_ptr(),
                columns_void_vec.len() as c_int,
            )
        }
        self
    }

    pub fn to_bool(&self, arg: bool) -> &Self {
        let ret = if arg { 1 } else { 0 } as *mut c_void;
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Bool as i32,
                ret,
                0.0,
                null(),
            )
        }
        self
    }

    pub fn to_u8(&self, arg: u8) -> &Self {
        let ret = arg as *mut c_void;
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                ret,
                0.0,
                null(),
            )
        }
        self
    }

    pub fn to_u16(&self, arg: u16) -> &Self {
        let ret = arg as *mut c_void;
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                ret,
                0.0,
                null(),
            )
        }
        self
    }

    pub fn to_i32(&self, arg: i32) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                arg as *mut c_void,
                0.0,
                null(),
            )
        }
        self
    }

    pub fn to_i64(&self, arg: i64) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                arg as *mut c_void,
                0.0,
                null(),
            )
        }
        self
    }

    pub fn to_f32(&self, arg: f32) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Double as i32,
                0 as *mut c_void,
                arg as c_double,
                null(),
            )
        }
        self
    }

    pub fn to_f64(&self, arg: f64) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                CPPType::Double as i32,
                0 as *mut c_void,
                arg as c_double,
                null(),
            )
        }
        self
    }

    pub fn to_string(&self, arg: Option<String>) -> &Self {
        match arg {
            None => unsafe {
                WCDBRustStatementUpdate_configValue(
                    self.get_cpp_obj(),
                    CPPType::Null as i32,
                    0 as *mut c_void,
                    0 as c_double,
                    null(),
                )
            },
            Some(str) => unsafe {
                WCDBRustStatementUpdate_configValue(
                    self.get_cpp_obj(),
                    CPPType::String as i32,
                    0 as *mut c_void,
                    0 as c_double,
                    str.to_cstring().as_ptr(),
                )
            },
        }
        self
    }

    pub fn to_expression_convertible<T>(&self, arg: &T) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configValue(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(arg),
                CppObject::get(arg),
                0 as c_double,
                null(),
            )
        }
        self
    }

    pub fn where_expression(&self, condition: &Expression) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configCondition(
                self.get_cpp_obj(),
                CppObject::get(condition.get_expression_operable()),
            );
        }
        self
    }

    pub fn order_by(&self, orders: &Vec<OrderingTerm>) -> &Self {
        if orders.is_empty() {
            return self;
        }
        let mut order_raw_vec = Vec::with_capacity(orders.len());
        for order in orders {
            order_raw_vec.push(order.get_cpp_obj());
        }
        unsafe {
            WCDBRustStatementUpdate_configOrders(
                self.get_cpp_obj(),
                order_raw_vec.as_ptr(),
                order_raw_vec.len(),
            );
        }
        self
    }

    pub fn limit_i64_i64(&self, from: i64, to: i64) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configLimitRange(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                from as c_long,
                CPPType::Int as i32,
                to as c_long,
            )
        }
        self
    }

    pub fn limit_i64_expression_convertible<T>(&self, from: i64, to: &T) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configLimitRange(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                from as c_long,
                Identifier::get_cpp_type(to),
                CppObject::get(to) as c_long,
            )
        }
        self
    }

    pub fn limit_expression_convertible<T>(&self, from: &T, to: &T) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configLimitRange(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(from),
                CppObject::get(from) as c_long,
                Identifier::get_cpp_type(to),
                CppObject::get(to) as c_long,
            )
        }
        self
    }

    pub fn limit_expression_convertible_i64<T>(&self, from: &T, to: i64) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configLimitRange(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(from),
                CppObject::get(from) as c_long,
                CPPType::Int as i32,
                to as c_long,
            )
        }
        self
    }

    pub fn limit(&self, count: i64) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configLimitCount(
                self.get_cpp_obj(),
                CPPType::Int as i32,
                count,
            );
        }
        self
    }

    pub fn limit_count_expression_convertible<T>(&self, count: &T) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configLimitCount(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(count),
                CppObject::get(count) as c_long,
            );
        }
        self
    }

    pub fn offset(&self, offset: i64) -> &Self {
        unsafe {
            WCDBRustStatementUpdate_configOffset(self.get_cpp_obj(), CPPType::Int as i32, offset);
        }
        self
    }

    pub fn offset_expression_convertible<T>(&self, offset: &T) -> &Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait + CppObjectTrait,
    {
        unsafe {
            WCDBRustStatementUpdate_configOffset(
                self.get_cpp_obj(),
                Identifier::get_cpp_type(offset),
                CppObject::get(offset) as c_long,
            );
        }
        self
    }
}
