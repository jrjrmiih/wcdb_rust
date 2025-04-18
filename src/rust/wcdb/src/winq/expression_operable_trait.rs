use crate::base::value::Value;
use crate::winq::expression::Expression;
use crate::winq::expression_convertible::ExpressionConvertibleTrait;
use crate::winq::identifier::IdentifierStaticTrait;
use crate::winq::identifier_convertible::IdentifierConvertibleTrait;

pub trait ExpressionOperableTrait {
    //todo dengxudong
    fn is_null(&self) -> Expression;

    fn not_null(&self) -> Expression;

    fn or<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn and<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn multiply_expression_convertible<T>(&mut self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn multiply_byte(&mut self, operand: i8) -> Expression;

    fn multiply_short(&mut self, operand: i16) -> Expression;

    fn multiply_int(&self, operand: i32) -> Expression;

    fn multiply_long(&mut self, operand: i64) -> Expression;

    fn multiply_float(&mut self, operand: f32) -> Expression;

    fn multiply_double(&mut self, operand: f64) -> Expression;

    fn divide_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn divide_byte(&self, operand: i8) -> Expression;

    fn divide_short(&self, operand: i16) -> Expression;

    fn divide_int(&self, operand: i32) -> Expression;

    fn divide_long(&self, operand: i64) -> Expression;

    fn divide_float(&self, operand: f32) -> Expression;

    fn divide_double(&self, operand: f64) -> Expression;

    fn mod_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn mod_byte(&self, operand: i8) -> Expression;

    fn mod_short(&self, operand: i16) -> Expression;

    fn mod_int(&self, operand: i32) -> Expression;

    fn mod_long(&self, operand: i64) -> Expression;

    fn mod_float(&self, operand: f32) -> Expression;

    fn mod_double(&self, operand: f64) -> Expression;

    fn add_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn add_byte(&self, operand: i8) -> Expression;

    fn add_short(&self, operand: i16) -> Expression;

    fn add_int(&self, operand: i32) -> Expression;

    fn add_long(&self, operand: i64) -> Expression;

    fn add_float(&self, operand: f32) -> Expression;

    fn add_double(&self, operand: f64) -> Expression;

    fn minus_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn minus_byte(&self, operand: i8) -> Expression;

    fn minus_short(&self, operand: i16) -> Expression;

    fn minus_int(&self, operand: i32) -> Expression;

    fn minus_long(&self, operand: i64) -> Expression;

    fn minus_float(&self, operand: f32) -> Expression;

    fn minus_double(&self, operand: f64) -> Expression;

    fn left_shift_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn left_shift_byte(&self, operand: i8) -> Expression;

    fn left_shift_short(&self, operand: i16) -> Expression;

    fn left_shift_int(&self, operand: i32) -> Expression;

    fn left_shift_long(&self, operand: i64) -> Expression;

    fn right_shift_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn right_shift_byte(&self, operand: i8) -> Expression;

    fn right_shift_short(&self, operand: i16) -> Expression;

    fn right_shift_int(&self, operand: i32) -> Expression;

    fn right_shift_long(&self, operand: i64) -> Expression;

    fn bit_and_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn bit_and_byte(&self, operand: i8) -> Expression;

    fn bit_and_short(&self, operand: i16) -> Expression;

    fn bit_and_int(&self, operand: i32) -> Expression;

    fn bit_and_long(&self, operand: i64) -> Expression;

    fn bit_or_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn bit_or_byte(&self, operand: i8) -> Expression;

    fn bit_or_short(&self, operand: i16) -> Expression;

    fn bit_or_int(&self, operand: i32) -> Expression;

    fn bit_or_long(&self, operand: i64) -> Expression;

    fn lt_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn lt_byte(&self, operand: i8) -> Expression;

    fn lt_short(&self, operand: i16) -> Expression;

    fn lt_int(&self, operand: i32) -> Expression;

    fn lt_long(&self, operand: i64) -> Expression;

    fn lt_double(&self, operand: f64) -> Expression;

    fn lt_string(&self, operand: &str) -> Expression;

    fn le_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn le_byte(&self, operand: i8) -> Expression;

    fn le_short(&self, operand: i16) -> Expression;

    fn le_int(&self, operand: i32) -> Expression;

    fn le_long(&self, operand: i64) -> Expression;

    fn le_float(&self, operand: f32) -> Expression;

    fn le_double(&self, operand: f64) -> Expression;

    fn le_string(&self, operand: &str) -> Expression;

    fn gt_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn gt_byte(&self, operand: i8) -> Expression;

    fn gt_short(&self, operand: i16) -> Expression;

    fn gt_int(&self, operand: i32) -> Expression;

    fn gt_long(&self, operand: i64) -> Expression;

    fn gt_float(&self, operand: f32) -> Expression;

    fn gt_double(&self, operand: f64) -> Expression;

    fn gt_string(&self, operand: &str) -> Expression;

    fn ge_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn ge_byte(&self, operand: i8) -> Expression;

    fn ge_short(&self, operand: i16) -> Expression;

    fn ge_int(&self, operand: i32) -> Expression;

    fn ge_long(&self, operand: i64) -> Expression;

    fn ge_float(&self, operand: f32) -> Expression;

    fn ge_double(&self, operand: f64) -> Expression;

    fn ge_string(&self, operand: &str) -> Expression;

    fn eq_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn eq_bool(&self, operand: bool) -> Expression;

    fn eq_byte(&self, operand: i8) -> Expression;

    fn eq_short(&self, operand: i16) -> Expression;

    fn eq_int(&self, operand: i32) -> Expression;

    fn eq_long(&self, operand: i64) -> Expression;

    fn eq_float(&self, operand: f32) -> Expression;

    fn eq_double(&self, operand: f64) -> Expression;

    fn eq_string(&self, operand: &str) -> Expression;

    fn not_eq_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_eq_bool(&self, operand: bool) -> Expression;

    fn not_eq_byte(&self, operand: i8) -> Expression;

    fn not_eq_short(&self, operand: i16) -> Expression;

    fn not_eq_int(&self, operand: i32) -> Expression;

    fn not_eq_long(&self, operand: i64) -> Expression;

    fn not_eq_float(&self, operand: f32) -> Expression;

    fn not_eq_double(&self, operand: f64) -> Expression;

    fn not_eq_string(&self, operand: &str) -> Expression;

    fn concat_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn concat_byte(&self, operand: i8) -> Expression;

    fn concat_short(&self, operand: i16) -> Expression;

    fn concat_int(&self, operand: i32) -> Expression;

    fn concat_long(&self, operand: i64) -> Expression;

    fn concat_float(&self, operand: f32) -> Expression;

    fn concat_double(&self, operand: f64) -> Expression;

    fn concat_string(&self, operand: &str) -> Expression;

    fn between_expr_expr<T>(&self, begin: &T, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_expr_long<T>(&self, begin: &T, end: i64) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_expr_double<T>(&self, begin: &T, end: f64) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_expr_string<T>(&self, begin: &T, end: &str) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_long_expr<T>(&self, begin: i64, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_long_long(&self, begin: i64, end: i64) -> Expression;

    fn between_long_double(&self, begin: i64, end: f64) -> Expression;

    fn between_long_string(&self, begin: i64, end: &str) -> Expression;

    fn between_double_expr<T>(&self, begin: i64, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_double_long(&self, begin: f64, end: i64) -> Expression;

    fn between_double_double(&self, begin: f64, end: f64) -> Expression;

    fn between_double_string(&self, begin: f64, end: &str) -> Expression;

    fn between_string_expr<T>(&self, begin: &str, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn between_string_long(&self, begin: &str, end: i64) -> Expression;

    fn between_string_double(&self, begin: &str, end: f64) -> Expression;

    fn between_string_string(&self, begin: &str, end: &str) -> Expression;

    fn not_between_expr_expr<T>(&self, begin: &T, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_expr_long<T>(&self, begin: &T, end: i64) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_expr_double<T>(&self, begin: &T, end: f64) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_expr_string<T>(&self, begin: &T, end: &str) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_long_expr<T>(&self, begin: i64, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_long_long(&self, begin: i64, end: i64) -> Expression;

    fn not_between_long_double(&self, begin: i64, end: f64) -> Expression;

    fn not_between_long_string(&self, begin: i64, end: &str) -> Expression;

    fn not_between_double_expr<T>(&self, begin: i64, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_double_long(&self, begin: f64, end: i64) -> Expression;

    fn not_between_double_double(&self, begin: f64, end: f64) -> Expression;

    fn not_between_double_string(&self, begin: f64, end: &str) -> Expression;

    fn not_between_string_expr<T>(&self, begin: &str, end: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn not_between_string_long(&self, begin: &str, end: i64) -> Expression;

    fn not_between_string_double(&self, begin: &str, end: f64) -> Expression;

    fn not_between_string_string(&self, begin: &str, end: &str) -> Expression;

    fn in_short(&self, operands: Vec<i16>) -> Expression;

    fn in_int(&self, operands: Vec<i32>) -> Expression;

    fn in_long(&self, operands: Vec<i64>) -> Expression;

    fn in_float(&self, operands: Vec<f32>) -> Expression;

    fn in_double(&self, operands: Vec<f64>) -> Expression;

    fn in_string(&self, operands: Vec<&str>) -> Expression;

    fn in_value(&self, operands: Vec<Value>) -> Expression;

    // todo dengxudong
    //public <T> Expression in(@NotNull Set<T> operands) {
    // public <T> Expression in(@NotNull List<T> operands) {

    fn not_in_short(&self, operands: Vec<i16>) -> Expression;

    fn not_in_int(&self, operands: Vec<i32>) -> Expression;

    fn not_in_long(&self, operands: Vec<i64>) -> Expression;

    fn not_in_float(&self, operands: Vec<f32>) -> Expression;

    fn not_in_double(&self, operands: Vec<f64>) -> Expression;

    fn not_in_string(&self, operands: Vec<&str>) -> Expression;

    fn not_in_value(&self, operands: Vec<Value>) -> Expression;

    // todo dengxudong
    // public <T> Expression notIn(@NotNull Set<T> operands)
    // public <T> Expression notIn(@NotNull List<T> operands)

    fn in_table(&self, table: &str) -> Expression;

    // Expression notInTable(@NotNull String table)
    // Expression inFunction(@NotNull String table)
    // public Expression notInFunction(@NotNull String table)
    // Expression in(@NotNull StatementSelect select)
    // Expression notIn(@NotNull StatementSelect select)

    fn collate(&self, collation: &str) -> Expression;

    fn substr_short(&self, start: i16, length: i16) -> Expression;

    fn substr_int(&self, start: i32, length: i32) -> Expression;

    fn substr_long(&self, start: i64, length: i64) -> Expression;

    fn like(&self, content: &str) -> Expression;

    fn not_like(&self, content: &str) -> Expression;

    fn glob(&self, content: &str) -> Expression;

    fn not_glob(&self, content: &str) -> Expression;

    fn match_string(&self, content: &str) -> Expression;

    fn not_match(&self, content: &str) -> Expression;

    fn regexp(&self, content: &str) -> Expression;

    fn not_regexp(&self, content: &str) -> Expression;

    fn is_bool(&self, operand: bool) -> Expression;

    fn is_byte(&self, operand: u8) -> Expression;

    fn is_short(&self, operand: i16) -> Expression;

    fn is_i32(&self, operand: i32) -> Expression;

    fn is_long(&self, operand: i64) -> Expression;

    fn is_float(&self, operand: f32) -> Expression;

    fn is_double(&self, operand: f64) -> Expression;

    fn is_string(&self, operand: &str) -> Expression;

    fn is_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn is_not_bool(&self, operand: bool) -> Expression;

    fn is_not_byte(&self, operand: u8) -> Expression;

    fn is_not_short(&self, operand: i16) -> Expression;

    fn is_not_i32(&self, operand: i32) -> Expression;

    fn is_not_long(&self, operand: i64) -> Expression;

    fn is_not_float(&self, operand: f32) -> Expression;

    fn is_not_double(&self, operand: f64) -> Expression;

    fn is_not_string(&self, operand: &str) -> Expression;

    fn is_not_expression_convertible<T>(&self, operand: &T) -> Expression
    where
        T: IdentifierStaticTrait + IdentifierConvertibleTrait + ExpressionConvertibleTrait;

    fn avg(&self) -> Expression;

    fn count(&self) -> Expression;

    fn group_concat(&self) -> Expression;

    fn group_concat_string(&self, sperator: &str) -> Expression;

    fn max(&self) -> Expression;

    fn min(&self) -> Expression;

    fn sum(&self) -> Expression;

    fn total(&self) -> Expression;

    fn abs(&self) -> Expression;

    fn hex(&self) -> Expression;

    fn length(&self) -> Expression;

    fn lower(&self) -> Expression;

    fn upper(&self) -> Expression;

    fn round(&self) -> Expression;

    fn match_info(&self) -> Expression;

    fn offsets(&self) -> Expression;

    fn snippet(&self) -> Expression;

    fn bm25(&self) -> Expression;

    fn highlight(&self) -> Expression;

    fn substring_match_info(&self) -> Expression;
}
