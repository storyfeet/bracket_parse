use ::*;

pub struct BracketIter<'a> {
    n:&'a Bracket,
    i:usize,
}

impl<'a> BracketIter<'a>{
    pub fn new(b:&'a Bracket)->Self{
        BracketIter{
            n:b,
            i:0,
        }
    }
}


impl<'a> Iterator for BracketIter<'a>{
    type Item = &'a Bracket;
         
    fn next(&mut self)->Option<&'a Bracket>{
        match self.n {
            Bracket::Branch(ref v)=>{
                if self.i >= v.len() {
                    return None;
                }
                self.i += 1;
                Some(&v[self.i-1])
            }
            _=> None,
        }
    }
}


#[cfg(test)]
mod iter_tests{
    use ::*;

    #[test]
    fn bracket_iter(){
        let bk = br().sib_lf("a").sib_lf("b").sib_lf("c");
        assert_eq!((&bk).into_iter().last().unwrap().match_str(),"c");
    }
}




