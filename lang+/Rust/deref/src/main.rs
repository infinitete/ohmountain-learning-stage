use std::ops::Deref;

enum Sex {
    Male,
    Female
}

struct Person {
    name: String,
    age: u8,
    sex: Sex
 }

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &String {
        &self.name
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Sex::Male => write!(f, "{}", "男"),
            &Sex::Female => write!(f, "{}", "女")
        }
    }
}

impl std::fmt::Debug for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Sex::Male => write!(f, "性别：{}", "男"),
            &Sex::Female => write!(f, "性别：{}", "女")
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Person {}\n", "{").unwrap();
        write!(f, "  name：{}\n", self.name).unwrap();
        write!(f, "  age ：{}\n", self.age).unwrap();

        match self.sex {
            Sex::Male => write!(f, "  sex ：{}\n", "男").unwrap(),
            Sex::Female => write!(f, "  sex ：{}\n", "女").unwrap()
        };

        write!(f, "{}", "}")
    }
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n{}\n", "------").unwrap();
        write!(f, "姓名：{}\n", self.name).unwrap();
        write!(f, "年龄：{}\n", self.age).unwrap();

        match self.sex {
            Sex::Male => write!(f, "性别：{}\n", "男").unwrap(),
            Sex::Female => write!(f, "性别：{}\n", "女").unwrap(),
        };

        write!(f, "{}\n", "------")
    }
}

fn main() {

    let person = Person {
        name: String::from("仁山"),
        age: 27,
        sex: Sex::Male
    };

    println!("{}, {}, {}\n", *person, person.age, person.sex);

    println!("{}", person);
    println!("{:?}", person);
}
