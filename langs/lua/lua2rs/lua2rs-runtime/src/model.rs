use fyrox::core::ImmutableString;
use std::collections::HashMap;

pub struct HeapPointer {}

pub enum Value {
    Reference(HeapPointer),
    String(ImmutableString),
    Function(Function),
    Number(f64),
    Bool(bool),
}

pub enum HeapAllocation {
    Table(TableAllocation),
    UserData(Box<dyn UserData>),
}

pub struct TableAllocation {
    pub last_used_index: u32,
    pub data: HashMap<ImmutableString, Value>,
}

pub struct Heap {
    pub last_pointer: HeapPointer,
    pub allocations: HashMap<HeapPointer, HeapAllocation>,
}

trait UserData: LuaObject {}

pub enum Function {
    Fn(fn(inputs: &[Value], outputs: &mut Vec<Value>)),
    Closure(Box<dyn Fn(Vec<Value>, &mut Vec<Value>)>),
}

trait LuaObject {
    fn get_by_index(&self, index: u32) -> Option<Value>;
    fn get_by_key(&self, key: ImmutableString) -> Option<Value>;
}
