pub fn run() {
    println!("generics.rs",);
    let person = Person::new("John".to_string(), 30);
    person.get_name();
    // get_name(person);
    // get_name_impl(person);
    // get_name_where(person);
    println!("{}", person.get_age_type());
}

struct Person<T> {
    name: String,
    age: T,
}

impl<T> Person<T> {
    fn new(name: String, age: T) -> Self {
        Person { name, age }
    }
}

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge<T> {
    fn get_age(&self) -> &T;
}

trait GetAgeType {
    type Age;
    fn get_age_type(&self) -> &Self::Age;
}

impl<T> GetAgeType for Person<T> {
    type Age = T;
    fn get_age_type(&self) -> &Self::Age {
        &self.age
    }
}

impl<T> GetAge<T> for Person<T> {
    fn get_age(&self) -> &T {
        &self.age
    }
}

impl GetName for Person<i32> {
    fn get_name(&self) -> &String {
        &self.name
    }
}

// トレイト境界サンプル
fn get_name<T: GetName>(person: T) {
    println!("{}", person.get_name());
}

// トレイト境界サンプル where句
fn get_name_where<T>(person: T)
where
    T: GetName,
{
    println!("{}", person.get_name());
}

// impl
fn get_name_impl(person: impl GetName) {
    println!("{}", person.get_name());
}
