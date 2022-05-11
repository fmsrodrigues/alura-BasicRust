// Global
const PI: f32 = 3.14;
static VARIAVEL: u8 = 1;

// unsafe global
static mut VARIAVEL2: u8 = 1;

fn main() {
  escopo();
  sombra();
  soma(3, 5);
  conditional();
  loops();
  matches();
  ownership();
  patternMatching();
  errors();
}

// ***************************
// 1
// ***************************

fn escopo() {
  let variavel: i32 = 128;
  let variavel: i32 = 127; // redeclaration
  println!(
    "Variavel = {}, tamanho = {} bytes",
    variavel,
    std::mem::size_of_val(&variavel)
  );

  let decimal: f32 = 2.5;
  println!("Decimal = {}", decimal);

  let boolean = false;
  println!(
    "Boolean = {}, tamanho = {}",
    boolean,
    std::mem::size_of_val(&boolean)
  );

  let mut character: char = 'C';
  character = 'F'; // mutate
  println!(
    "Character = {}, tamanho = {}",
    character,
    std::mem::size_of_val(&character)
  );

  println!("PI = {}", PI);
  // Avoid unsafe!
  unsafe {
    println!("variavel global = {}", VARIAVEL2);
  }
}

fn sombra() {
  let a = 123;

  {
    let b = 456;
    println!("b = {}", b);

    let a = 777; // shadowing
    println!("a = {}", a);
  }

  println!("a = {}", a);
  // doens't work
  // println!("b = {}", b);
}

fn soma(a: i32, b: i32) -> i32 {
  println!("{} + {} = {}", a, b, a + b);

  // return a + b
  a + b // without the ';' this value will be returned. this is an expression.
}

// ***************************
// 2
// ***************************

fn conditional() {
  let age: u8 = 36;

  if (age > 18) {
    println!("Just getting started, huh?");
  } else if age > 30 && age < 60 {
    // it's possible to ommit the parentheses
    println!("Still young, buddy!");
  } else {
    println!("Too young, I'm sorry!");
  }

  // without the ';' those strings will be returned as they are an exprewsion.
  let isAllowedToDrinkBeer = if age > 18 { "allowed" } else { "not allowed" };
  println!("Drink beers: {}", isAllowedToDrinkBeer);
}

// ***************************
// 3
// ***************************

fn loops() {
  let multiplicator: u8 = 5;
  let mut counter: u8 = 0;

  while counter < 5 {
    counter += 1;

    if counter == 2 {
      continue; // skip the rest of iteration
    }

    println!(
      "{} x {} = {}",
      multiplicator,
      counter,
      multiplicator * counter
    );
  }

  counter = 0;
  loop {
    // this is infinite loop
    counter += 1;

    if counter == 2 {
      continue; // skip the rest of iteration
    }

    println!(
      "{} x {} = {}",
      multiplicator,
      counter,
      multiplicator * counter
    );

    if counter > 5 {
      break;
    }
  }

  for i in 1..=5 {
    // it's the same as 1..6
    println!("{} x {} = {}", multiplicator, i, multiplicator * i);
  }
}

// ***************************
// 4
// ***************************

fn matches() {
  let language = "Python";
  let porpouse = match language {
    "Php" => "Web",
    "Kotlin" => "Android",
    "Python" => "Data Science",
    _ => "Unknown",
  };

  println!("The language {} porpouse is {}", language, porpouse);
}

fn ownership() {
  // let stringEx = String::from("Name");
  let mut stringEx = String::from("Name");
  // stealString(stringEx); // stringEx is invalid after this line, stealString is the new owner
  // stealString(&stringEx); // Borrowing
  stealString(&mut stringEx); // Borrowing
  println!("{}", stringEx);
}

// fn stealString(string: String) {
// fn stealString(string: &String) { // Borrowing and inmutable
fn stealString(string: &mut String) {
  string.push_str(" Surname");
  println!("{}", string);
}

// ***************************
// 5
// ***************************

fn patternMatching() {
  for x in 1..=20 {
    println!(
      "{}:{}",
      x,
      match x {
        1 => "too little",
        2 | 3 => "a little",
        4..=10 => "a lot",
        _ if x % 2 == 0 => "just enough",
        _ => "too much",
      }
    );
  }

  let point = (0, 0);
  match point {
    (0, 0) => "origin",
    (0, _) => "x axis",
    (_, 0) => "y axis",
    (_, _) => "out of axis",
  };
}

fn errors() {
  // panic!("Erro proposital!"); // irrecoverable

  match results() {
    Ok(s) => println!("Success: {}", s),
    Err(n) => println!("Error code: {}", n),
  }
}

fn results() -> Result<String, u8> {
  // Err(String::from(42))
  Ok(String::from("Ok!"))
}
