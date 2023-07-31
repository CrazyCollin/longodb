use std::sync::Weak;
use crate::db::db::InnerDB;

pub struct Collection<'a,T:'a>{
    name:String,
    db:Weak<InnerDB>,
}

impl<'a,T> Collection<'_,T> {

    pub fn new(name:&str,db:Weak<InnerDB>)->Collection<T>{
        Self{
            db,
            name:name.into(),
        }
    }
}