fn main() {
  array();
  matrixes();
  enums();
  options();
  vectors();
  accounts();
}

// ***************************
// 1
// ***************************

fn array() {
  // let grade1: f32 = 10f32; // casting
  // let grade2: f32 = 9.5; // casting

  // let grades: [f32; 4] = [6.5; 4] // [6.5, 6.5, 6.5, 6.5]
  let grades: [f32; 4] = [10.1, 8.0, 9.5, 6.0];
  // let position: u32 = 0; // not optimized for every system
  let position: usize = 0;

  println!("grade: {}", grades[position]);

  for grade in grades {
    println!("grades: {}", grade);
  }

  for index in 0..grades.len() {
    println!("A grade {} é = {}", index + 1, grades[index]);
  }
}

fn matrixes() {
  let matrix: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

  for line in matrix {
    for item in line {
      println!("Item = {}", item);
    }
  }
}

// ***************************
// 2
// ***************************

fn enums() {
  println!("Is weekend? {}", isWeekend(DayOfTheWeek::Saturday));
  colors();
}

#[allow(dead_code)]
enum DayOfTheWeek {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

fn isWeekend(day: DayOfTheWeek) -> bool {
  match day {
    DayOfTheWeek::Sunday | DayOfTheWeek::Saturday => true,
    _ => false,
  }
}

// Meta attribute
#[allow(dead_code)]
enum Color {
  Red,
  Green,
  Blue,
  RgbColor(u8, u8, u8),
  CymkColor {
    cyan: u8,
    magenta: u8,
    yellow: u8,
    black: u8,
  },
}

fn colors() {
  // let color = Color::RgbColor(0, 0, 0);
  let color = Color::CymkColor {
    cyan: 0,
    magenta: 50,
    yellow: 70,
    black: 255,
  };

  println!(
    "Cor = {}",
    match color {
      Color::Red => "red",
      Color::Green => "green",
      Color::Blue => "blue",
      Color::RgbColor(0, 0, 0)
      | Color::CymkColor {
        cyan: _,
        magenta: _,
        yellow: _,
        black: 255,
      } => "Black",
      Color::RgbColor(_, _, _) => "unknown RGB",
      Color::CymkColor {
        cyan: _,
        magenta: _,
        yellow: _,
        black: _,
      } => "unknwon CYMK",
    }
  )
}

// ***************************
// 3
// ***************************

fn options() {
  let fileContent = readFile(String::from(""));
  match &fileContent {
    Some(value) => println!("{}", value),
    None => println!("File not found"),
  }

  // Debug value
  println!("{:?}", fileContent);

  // pattern matching
  if let Some(valor) = fileContent {
    println!("File has content!")
  }

  // runs while Some(value) is true
  // while let Some(value) = fileContent {}
}

fn readFile(path: String) -> Option<String> {
  Some(String::from("File content"))
}

// Generics
// Enum template
enum CustomOption<T> {
  Some(T),
  None,
}

enum CustomResult<S, E> {
  Ok(S),
  Err(E),
}

// ***************************
// 4
// ***************************

fn vectors() {
  // let mut grades: Vec<f32> = Vec::new();
  // grades.push(10.0);
  // grades.push(9.7);
  // grades.push(2.2);

  let mut grades: Vec<f32> = vec![10.0, 8.0, 3.2];
  println!("Vector capacity: {}", grades.capacity());

  grades.push(4.5);
  println!("Vector capacity: {}", grades.capacity());

  println!("{:?}", grades);
  println!("1st Grade: {}", grades[0]);
  println!(
    "7th Grade: {}",
    match grades.get(7) {
      // Some(&grade) => grade,
      Some(grade) => *grade,
      None => -1.0,
    }
  );

  // Borrow grades value
  for grade in &grades {
    println!("grade: {}", grade);
  }

  while let Some(grade) = grades.pop() {
    println!("Last value: {}", grade);
    println!("{:?}", grades);
  }
  println!("Vector capacity: {}", grades.capacity());

  // this will never increase the capacity and avoid memory allocation which is heavily demanding
  let mut gradesWithCapacity: Vec<f32> = Vec::with_capacity(32);
  println!(
    "Vector with Capacity capacity: {}",
    gradesWithCapacity.capacity()
  );
}

// ***************************
// 5
// ***************************

struct Owner {
  name: String,
  surname: String,
}

struct Account {
  owner: Owner,
  balance: f64,
}

// it's a method for your struct (object), but must receive self as 1st parameter as a mutable reference
impl Account {
  fn withdraw(&mut self, value: f64) {
    self.balance -= value;
  }
}

fn accounts() {
  let owner = Owner {
    name: String::from("Doe"),
    surname: String::from("Doe"),
  };

  let mut account: Account = Account {
    owner, // you can omit the right side of the assignment, like JS
    balance: 64.5,
  };

  println!(
    "Owner: {} {}, Balance: {}",
    account.owner.name, account.owner.surname, account.balance
  );

  account.withdraw(10.0);

  println!(
    "Owner: {} {}, Balance: {}",
    account.owner.name, account.owner.surname, account.balance
  );
}
