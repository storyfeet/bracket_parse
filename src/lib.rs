use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub enum Bracket{
    Branch(Vec<Bracket>),
    Leaf(String),
    Empty,
}

pub fn lf(s:&str)->Bracket{
    Bracket::Leaf(s.to_string())
}

pub fn br()->Bracket{
    Bracket::Branch(Vec::new())
}




impl FromStr for Bracket{
    type Err = String;
    fn from_str(s:&str)->Result<Bracket,String>{
        let mut res = Bracket::Empty;
        let mut it = s.chars();
        let mut curr = String::new();
        while let Some(c) = it.next() {
            Bracket::match_char(c,&mut it,&mut curr,&mut res)?;
        }
        if curr.len() >0 {
            res.add_sib_str(curr);
        }
        Ok(res)
    }
}

impl Bracket{
    fn add_sib_str(&mut self,s:String){
        if s.len() == 0 {
            return
        }
        self.add_sibling(Bracket::Leaf(s));
    }

    /// chaining method for quickly creating a tree Adds a sibling to a bracket
    /// if it is a leaf makes it a parent.
    pub fn sib(mut self,s:Self)->Self{
        self.add_sibling(s);
        self
    }

    /// chainging method for easily adding a leaf as a sibling from an &str 
    pub fn sibs(self,s:&str)->Self{
        self.sib(lf(s))
    }

    fn add_sibling(&mut self,s:Bracket){
        if s == Bracket::Empty {
            return
        }

        let c:String = match self {
            Bracket::Branch(ref mut v)=>{
                v.push(s);
                return
            }
            Bracket::Empty=>{
                *self = s;
                return 
            }
            Bracket::Leaf(content)=>content.to_string(),
        };

        *self = Bracket::Branch(vec![Bracket::Leaf(c),s]);
    }

    
    fn match_char<I>(c:char,it:&mut I,curr:&mut String,res:&mut Bracket)->Result<(),String>
        where I:Iterator<Item=char>{
            match c {
                '('=>{  // When Non Lexical Lifetimes comes, we can get rid of these curr.clone()s hopefully
                    res.add_sib_str(curr.clone());
                    *curr = String::new();
                    res.add_sibling(Bracket::from_bracket(it,')')?);
                },
                '{'=>{ //Todo make Json-esque prob needs Object Variant 
                    res.add_sib_str(curr.clone());
                    *curr = String::new();
                    res.add_sibling(Bracket::from_bracket(it,'}')?);
                },
                '['=>{
                    res.add_sib_str(curr.clone());
                    *curr = String::new();
                    res.add_sibling(Bracket::from_bracket(it,']')?);
                },
                '"'|'\''=>{
                    res.add_sib_str(curr.clone());
                    *curr = String::new();
                    res.add_sibling(Bracket::from_quotes(it,c)?);
                }
                ' '|','=>{
                    res.add_sib_str(curr.clone());
                    *curr = String::new();
                },
                other=>curr.push(other),
            }
            Ok(())
    }


    fn from_bracket<I:Iterator<Item=char>>(it:&mut I,delim:char)->Result<Bracket,String>{
        let mut res = Bracket::Branch(Vec::new());
        let mut curr = String::new();
        while let Some(c) = it.next() {
            if c == delim {
                res.add_sib_str(curr.clone());
                return Ok(res);
            }
            Bracket::match_char(c,it,&mut curr,&mut res)?;
        }
        Err(format!("Close Delim '{}' not found",delim))
    }

    fn from_quotes<I:Iterator<Item=char>>(it:&mut I,delim:char)->Result<Bracket,String>{
        let mut curr = String::new();
        while let Some(c) = it.next() {
            if c == delim {
                return Ok(Bracket::Leaf(curr));
            }
            match c {
                '\\'=>{
                    match it.next(){
                        Some(c2)=>{
                            curr.push(c2);
                            continue
                        },
                        None=>return Err("Escape before end of string".to_string()),
                    }
                },
                _=> curr.push(c),
            }
        }
        Err(format!("Close Delim '{}' not found",delim))
    }
}





#[cfg(test)]
mod tests {
    use super::{Bracket,br,lf};
    use std::str::FromStr;
    #[test]
    fn spaces() {
        let b1 = Bracket::from_str("matt dave (andy steve)").unwrap();
        let c1 = br().sibs("matt").sibs("dave").sib(
                            br().sibs("andy").sibs("steve")
                        );
                                        
        let b2 = Bracket::from_str("matt dave( andy steve)").unwrap();
        let b3 = Bracket::from_str(" matt   dave  (   andy    steve  )  ").unwrap();
        assert_eq!(b1,c1);
        assert_eq!(b1,b2);
        assert_eq!(b1,b3);
    }

    #[test]
    fn empty_parent(){
        let b1 = Bracket::from_str("matt () dave").unwrap();
        let c1 = br().sibs("matt").sib(br()).sibs("dave");
        assert_eq!(b1,c1);
    }

    #[test]
    fn many_parent(){
        let b1 = Bracket::from_str("matt ({[() ()]})").unwrap();
        let c1 = lf("matt")
                    .sib(
                        br().sib(
                            br().sib(
                                br().sib(br()).sib(br())
                            )
                        )
                    );
        assert_eq!(b1,c1);

    }

    #[test]
    fn strings(){
        let b1 = Bracket::from_str(r#"matt"dave""#).unwrap();
        let c1 = br().sibs("matt").sibs("dave");

        assert_eq!(b1,c1);

        let b2 = Bracket::from_str(r#""andy \"hates\" cheese""#).unwrap();
        let c2 = lf(r#"andy "hates" cheese"#);
        assert_eq!(b2,c2);
    }

    #[test]
    fn errors(){
        assert!(Bracket::from_str("peop ( er").is_err());
        assert!(Bracket::from_str(r#""poop"#).is_err());
    }
}
