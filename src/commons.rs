// in this module common structs and enums are held
// due to its importance for the default packages it shall be included in the default install

pub struct UMinput{
    pub a : i64,
    pub b : i64,
    pub op : String
}

impl UMinput {
     fn clear_a(&mut self) -> i64 {
        self.a = 0;
        self.a
     }
     fn clear_b(&mut self) -> i64 {
        self.b = 0;
        self.b
     }
 }

pub struct Opstrings{
    pub sum : String,
    pub minus : String,
    pub times : String,
    pub quatient : String
}

impl Opstrings {
    fn build() -> Opstrings{
        Opstrings { sum: ("+".to_ascii_lowercase()), minus: ("-".to_ascii_lowercase()), times: ("*".to_ascii_lowercase()), quatient: ("/".to_ascii_lowercase()) }
    }
}

