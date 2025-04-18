use crate::base::cpp_object::CppObjectTrait;
use crate::winq::expression_convertible::ExpressionConvertibleTrait;
use crate::winq::identifier::{CPPType, Identifier, IdentifierStaticTrait, IdentifierTrait};
use std::ffi::{c_int, c_void};

extern "C" {
    fn WCDBRustOrderingTerm_create(cpp_type: c_int, expression: *mut c_void) -> *mut c_void;

    fn WCDBRustOrderingTerm_configOrder(cpp_obj: *mut c_void, order: c_int);
}

pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug)]
pub struct OrderingTerm {
    identifier: Identifier,
}

impl CppObjectTrait for OrderingTerm {
    fn set_cpp_obj(&mut self, cpp_obj: *mut c_void) {
        self.identifier.set_cpp_obj(cpp_obj);
    }

    fn get_cpp_obj(&self) -> *mut c_void {
        self.identifier.get_cpp_obj()
    }

    fn release_cpp_object(&mut self) {
        self.identifier.release_cpp_object();
    }
}

impl IdentifierTrait for OrderingTerm {
    fn get_description(&self) -> String {
        self.identifier.get_description()
    }
}

impl IdentifierStaticTrait for OrderingTerm {
    fn get_type() -> i32 {
        CPPType::OrderingTerm as i32
    }
}

impl OrderingTerm {
    pub fn new<T>(expression: &T) -> Self
    where
        T: ExpressionConvertibleTrait + IdentifierStaticTrait,
    {
        let left_cpp_obj = expression.as_cpp_object();
        let left_cpp_type = Identifier::get_cpp_type(expression);
        let cpp_obj = unsafe { WCDBRustOrderingTerm_create(left_cpp_type, left_cpp_obj) };
        let identifier = Identifier::new_with_obj(cpp_obj);
        OrderingTerm { identifier }
    }
}

impl OrderingTerm {
    pub fn order(self, order: Order) -> Self {
        unsafe {
            WCDBRustOrderingTerm_configOrder(self.get_cpp_obj(), (order as i32) + 1);
        }
        self
    }
}
