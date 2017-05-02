use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

fn main() {
    let name1 = Name {
        first_name: Some("仁山".to_string()),
        last_name: "田".to_string(),
    };

    let mut name2 = Name {
        first_name: None,
        last_name: "田".to_string(),
    };

    println!("{}", name1);
    println!("{}", &name2);

    name2.set_first_name("田".to_string());
    name2.set_last_name("仁山".to_string());

    println!("{}", &name2);
}


struct Name<T> {
    first_name: Option<T>,
    last_name: T,
}

impl Display for Name<String> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {

        match self.first_name.as_ref() {
            Some(data) => {
                return write!(f,
                              "Name {{first_name: {}, last_name: {} }}",
                              data.clone(),
                              self.last_name);
            }
            None => {
                return write!(f,
                              "Name {{first_name: None, last_name: {} }}",
                              self.last_name);
            }
        }
    }
}

trait SetName {
    fn set_first_name(&mut self, first_name: String) -> &mut Name<String>;
    fn set_last_name(&mut self, last_name: String) -> &mut Name<String>;
}

impl SetName for Name<String> {
    fn set_first_name(&mut self, first_name: String) -> &mut Name<String> {
        self.first_name = Some(first_name.clone());

        self
    }

    fn set_last_name(&mut self, last_name: String) -> &mut Name<String> {
        self.last_name = last_name.clone();

        self
    }
}
