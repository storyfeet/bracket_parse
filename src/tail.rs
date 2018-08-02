use std::iter::IntoIterator;
use super::Bracket;

pub const EMPTY_BRACKET:Bracket = Bracket::Empty;
pub const EMPTY_B_SLICE:[Bracket;0] = [];


pub enum Tail<'a>{
    Rest(&'a [Bracket]),
    Empty,
}

impl<'a> Tail<'a>{
    pub fn head(&'a self)->&'a Bracket{
        match self{
            Tail::Rest(v)=>match v.len(){
                0=>&EMPTY_BRACKET,
                _=>&v[0],
            },
            Tail::Empty=>&EMPTY_BRACKET,
        }
    }

    pub fn tail(&'a self)->Tail<'a>{
        match self{
            Tail::Rest(v)=>match v.len(){
                0|1=>Tail::Empty,
                _=>Tail::Rest(&v[1..]),
            },
            Tail::Empty=>Tail::Empty,
        }
    }
    pub fn head_tail(&'a self)->(&'a Bracket,Tail<'a>){
        (self.head(),self.tail()) 
    }

}

impl<'a> IntoIterator for Tail<'a>{
    type Item=&'a Bracket;
    type IntoIter = ::std::slice::Iter<'a,Bracket>;
    fn into_iter(self)->Self::IntoIter{
        match self {
            Tail::Rest(v)=>v.into_iter(),
            _=>EMPTY_B_SLICE.into_iter(),
                  
        }
    }
}

