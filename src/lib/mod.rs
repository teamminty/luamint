use std::{collections::HashMap, fmt::Display, hash::Hash};

/// The luamint struct, which defines the luamint environment, stack, globals, etc.
pub struct LuaMint {
    pub globals: LuaTable
}

impl LuaMint {
    
}

#[derive(Eq, Hash, PartialEq)]
pub enum Value {
    Nil,
    Number(i32),
    Boolean(bool),
    String(String),
    Function(LuaFunction),
    Userdata(Box<dyn LuaUserdata>),
    Thread,
    Table(LuaTable)
}

#[derive(Eq, Hash, PartialEq)]
pub struct LuaTable {
    value: HashMap<Value, Value>
}

impl LuaTable {
    pub fn get<T: IntoLuaValue>(&self, index: &T) -> Option<Value> {
        return self.value.get(&index.into_lua_value())
    }
}

pub trait IntoLuaValue {
    fn into_lua_value(&self) -> Value;
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Userdata(v) => {
                write!(f, "{}", v)
            },
            Nil => {
                write!(f, "nil")
            },
            Number(n) => {
                write!(f, "{}", n)
            },
            String(s) => {
                write!(f, "{}", s)
            },
            Function(v) => {
                write!(f, "function")
            },
            Table(v) => {
                todo!("Write printing for tables");
            },
            Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            },
            Thread => {
                todo!()
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct LuaFunction {

}

pub trait LuaUserdata: std::fmt::Display + Hash + Eq + PartialEq + Sized {

}