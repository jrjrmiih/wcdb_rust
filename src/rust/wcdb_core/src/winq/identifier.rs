use std::ffi::c_void;

use crate::base::cpp_object::{CppObject, CppObjectTrait};

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum CPPType {
    Invalid = 0,
    Null = 1,
    Bool = 2,
    Int = 3,
    UInt = 4,
    Double = 5,
    String = 6,

    Column = 7,
    Schema = 8,
    ColumnDef = 9,
    ColumnConstraint = 10,
    Expression = 11,
    LiteralValue = 12,
    ForeignKeyClause = 13,
    BindParameter = 14,
    RaiseFunction = 15,
    WindowDef = 16,
    Filter = 17,
    IndexedColumn = 18,
    TableConstraint = 19,
    CommonTableExpression = 20,
    QualifiedTableName = 21,
    OrderingTerm = 22,
    UpsertClause = 23,
    Pragma = 24,
    JoinClause = 25,
    TableOrSubquery = 26,
    JoinConstraint = 27,
    SelectCore = 28,
    ResultColumn = 29,
    FrameSpec = 30,

    AlterTableSTMT = 31,
    AnalyzeSTMT = 32,
    AttachSTMT = 33,
    BeginSTMT = 34,
    CommitSTMT = 35,
    RollbackSTMT = 36,
    SavepointSTMT = 37,
    ReleaseSTMT = 38,
    CreateIndexSTMT = 39,
    CreateTableSTMT = 40,
    CreateTriggerSTMT = 41,
    SelectSTMT = 42,
    InsertSTMT = 43,
    DeleteSTMT = 44,
    UpdateSTMT = 45,
    CreateViewSTMT = 46,
    CreateVirtualTableSTMT = 47,
    DetachSTMT = 48,
    DropIndexSTMT = 49,
    DropTableSTMT = 50,
    DropTriggerSTMT = 51,
    DropViewSTMT = 52,
    PragmaSTMT = 53,
    ReindexSTMT = 54,
    VacuumSTMT = 55,
    ExplainSTMT = 56,
}

pub trait IdentifierTrait {
    fn get_type() -> i32;
}

pub fn get_cpp_type<T: IdentifierTrait>(_: &T) -> i32 {
    T::get_type()
}

pub struct Identifier {
    cpp_obj: CppObject,
}

impl CppObjectTrait for Identifier {
     fn set_cpp_obj(&mut self, cpp_obj: *mut c_void) {
        self.cpp_obj.set_cpp_obj(cpp_obj);
    }

     fn get_cpp_obj(&self) -> *mut c_void {
        self.cpp_obj.get_cpp_obj()
    }

    fn release_cpp_object(&mut self) {
        self.cpp_obj.release_cpp_object();
    }
}

impl Identifier {
    pub fn new_with_obj(cpp_obj: *mut c_void) -> Self {
        Identifier {
            cpp_obj: CppObject::new_with_obj(cpp_obj),
        }
    }

    pub fn get_type(&self) -> i32 {
        0
    }

    pub fn get_cpp_type(identifier: &Identifier) -> i32 {
        identifier.get_type()
    }
}