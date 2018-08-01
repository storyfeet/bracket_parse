use std::str::FromStr;

#[derive(PartialEq,Debug)]
enum Bracket{
    Parent(Vec<Bracket>),
    Leaf(String),
    Empty,
}

impl FromStr for Bracket{
    type Err = String;
    fn from_str(s:&str)->Result<Bracket,String>{
        let mut res = Bracket::Empty;
        let mut it = s.chars();
        let mut curr = String::new();
        while let Some(c) = it.next() {
            match c {
                '('=>{
                    res.add_sib_str(curr.clone());
                    res.add_sibling(Bracket::from_bracket(&mut it,')')?);
                },
                ' '|','=>{
                    res.add_sib_str(curr);
                    curr = String::new();
                },
                other=>curr.push(other),
            }
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

    fn add_sibling(&mut self,s:Bracket){
        if s == Bracket::Empty {
            return
        }

        let c:String = match self {
            Bracket::Parent(ref mut v)=>{
                v.push(s);
                return
            }
            Bracket::Empty=>{
                *self = s;
                return 
            }
            Bracket::Leaf(content)=>content.to_string(),
        };

        *self = Bracket::Parent(vec![Bracket::Leaf(c),s]);
    }


    fn from_bracket<I:Iterator<Item=char>>(it:&mut I,delim:char)->Result<Bracket,String>{
        let mut res = Bracket::Empty;
        let mut curr = String::new();
        while let Some(c) = it.next() {
            if c == delim {
                res.add_sib_str(curr.clone());
                return Ok(res);
            }
            match c {
                '('=>{
                    res.add_sib_str(curr.clone());
                    res.add_sibling(Bracket::from_bracket(it,')')?);
                },
                ' '|','=>{
                    res.add_sib_str(curr);
                    curr = String::new();
                },
                other=>curr.push(other),
            }
        }
        Err(format!("Close Delim '{}' not found",delim))
    }
}





#[cfg(test)]
mod tests {
    use super::Bracket;
    use super::Bracket::*;
    use std::str::FromStr;
    #[test]
    fn it_works() {
        let b1 = Bracket::from_str("matt dave (andy steve)").unwrap();
        assert_eq!(b1,Parent(vec![Leaf("matt".to_string())
                                  ,Leaf("dave".to_string())
                                  ,Parent(vec![Leaf("andy".to_string())
                                                ,Leaf("steve".to_string())]
                                            )]
                                        ));
                                        
        let b2 = Bracket::from_str("matt dave( andy steve)").unwrap();
        assert_eq!(b1,b2);
        //let b2 = Bracket::from_str("matt dave andy steve");
        //let b3 = Bracket::from_str(r#""matt dave" "andy steve""#);
    }
}
