// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// // println!("Hello world!");
// // }
// //////////////////////////////////////////////////////////////////////////////////
// fn main() {
//     println!("{}, {}!", "Hello", "world"); // Hello, world!
//     println!("{0}, {1}!", "Hello", "world"); // Hello, world!
//     println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!
    
//     println!("{:?}", [1,2,3]); // [1, 2, 3]
//     println!("{:#?}", [1,2,3]);
//     /*
//         [
//             1,
//             2,
//             3
//         ]
//     */
    
//     // 🔎 format! macro is used to store the formatted STRING
//     let x = format!("{}, {}!", "Hello", "world");
//     println!("{}", x); // Hello, world!
    
//     let y = String::from("Hello, ") + "world!";
//     println!("{}", y); // Hello, world!
// }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = true;
// //   let b: bool = true;

// //   let (x, y) = (1, 2);

// //   let mut z = 5;
// //   z = 6;

// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //     println!("Hello, world!");
// // }
// // fn print_sum(a: i8, b: i8) {
// //     println!("sum is: {}", a + b);
// // }

// // // 01. Without the return keyword. Only last expression returns.
// // fn plus_one(a: i32) -> i32 {
// //   a + 1
// //   // There is no ending ; in the above line. It means this is an expression which equals to `return a+1;`
// // }

// // // 02. With the return keyword.
// // fn plus_two(a: i32) -> i32 {
// //   return a + 2; // Returns a+2. But, this's a bad practice.
// //     // Should use only on conditional returns, except in the last expression
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let x = true;
// //   let y: bool = false;

// //   // ⭐️ no TRUE, FALSE, 1, 0
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let x = 'x';
// //   let y = '😎';

// // // ⭐️ no "x", only single quotes
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
// //   let mut b = [1, 2, 3];

// //   let c: [i32; 0] = []; //[Type; NO of elements] -> [] /empty array
// //   let d: [i32; 3] = [1, 2, 3];

// //   let e = ["my value"; 3]; //["my value", "my value", "my value"];

// //   println!("{:?}", a); //[1, 2, 3]
// //   println!("{:#?}", a);
// //   //  [
// //   //      1,
// //   //      2,
// //   //      3
// //   //  ] 
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = (1, 1.5, true, 'a', "Hello wolrd!");
// //   // a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello world!"

// //   let b: (i32, f64) = (1, 1.5);

// //   let (c, d) = b; // c = 1, d = 1.5
// //   let (e, _, _, _, f) = a; //e = 1, f = "Hello, world!", _ indicates not interested of that item

// //   let g = (0,); //single-element tuple

// //   let h = (b, (2, 4), 5); //((1, 1.5), (2, 4), 5)

// //   println!("{:?}", a); //(1, 1.5, true, 'a', "Hello, world!")
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a: [i32; 4] = [1, 2, 3, 4];//Parent Array

// //   let b: &[i32] = &a; //Slicing whole array
// //   let c = &a[0..4]; // From 0th position to 4th(excluding)
// //   let d = &a[..]; //Slicing whole array

// //   let e = &a[1..3]; //[2, 3]
// //   let f = &a[1..]; //[2, 3, 4]
// //   let g = &a[..3]; //[1, 2, 3]
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = "Hello, world."; //a: &'static str
// //   let b: &str = "NIda Khan!";
// // }

// // fn main() {
// //   fn plus_one(a: i32) -> i32 {
// //     a + 1
// // }

// // let b: fn(i32) -> i32 = plus_one;
// // let c = b(5); //6
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = 5;
// //   let b = a + 1; //6
// //   let c = a - 1; //4
// //   let d = a * 2; //10
// //   let e = a / 2; // ⭐️ 2 not 2.5
// //   let f = a % 2; //1

// //   let g = 5.0 / 2.0; //2.5
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = 1;
// //   let b = 2;

// //   let c = a == b; //false
// //   let d = a != b; //true
// //   let e = a < b; //true
// //   let f = a > b; //false
// //   let g = a <= a; //true
// //   let h = a >= a; //true

// //   // 🔎
// //   let i = true > false; //true
// //   let j = 'a' > 'A'; //true
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = true;
// //   let b = false;

// //   let c = !a; //false
// //   let d = a && b; //false
// //   let e = a || b; //true
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = true;
// //   let b = false;

// //   let c = !a; //false
// //   let d = a && b; //false
// //   let e = a || b; //true
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = !-2; //1
// //   let b = !-1; //0
// //   let c = !0; //-1
// //   let d = !1; //-2
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = 1;
// //   let b = 2;

// //   let c = a & b;  //0  (01 && 10 -> 00)
// //   let d = a | b;  //3  (01 || 10 -> 11)
// //   let e = a ^ b;  //3  (01 != 10 -> 11)
// //   let f = a << b; //4  (Add b number of 0s to the end of a -> '01'+'00' -> 100)
// //   let g = a >> b; //0  (Remove b number of bits from the end of a -> o̶1̶ -> 0)
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let mut a = 2;

// //   a += 5; //2 + 5 = 7
// //   a -= 2; //7 - 2 = 5
// //   a *= 5; //5 * 5 = 25
// //   a /= 2; //25 / 2 = 12 not 12.5
// //   a %= 5; //12 % 5 = 2

// //   a &= 2; //10 && 10 -> 10 -> 2
// //   a |= 5; //010 || 101 -> 111 -> 7
// //   a ^= 2; //111 != 010 -> 101 -> 5
// //   a <<= 1; //'101'+'0' -> 1010 -> 10
// //   a >>= 2; //101̶0̶ -> 10 -> 2
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let a = 15;
// //   let b = (a as f64) / 2.0; //7.5
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   // Simplest Example
// //   let team_size = 7;
// //   if team_size < 5 {
// //       println!("Small");
// //   } else if team_size < 10 {
// //       println!("Medium");
// //   } else {
// //       println!("Large");
// //   }

// //   // partially refactored code
// //   let team_size = 7;
// //   let team_size_in_text;
// //   if team_size < 5 {
// //       team_size_in_text = "Small";
// //   } else if team_size < 10 {
// //       team_size_in_text = "Medium";
// //   } else {
// //       team_size_in_text = "Large";
// //   }
// //   println!("Current team size : {}", team_size_in_text);

// //   //optimistic code
// //   let team_size = 7;
// //   let team_size_in_text = if team_size < 5 {
// //       "Small" //⭐️no ;
// //   } else if team_size < 10 {
// //       "Medium"
// //   } else {
// //       "Large"
// //   };
// //   println!("Current team size : {}", team_size_in_text);


// //   let is_below_eighteen = if team_size < 18 { true } else { false };
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let tshirt_width = 20;
// //   let tshirt_size = match tshirt_width {
// //       16 => "S", // check 16
// //       17 | 18 => "M", // check 17 and 18
// //       19 ... 21 => "L", // check from 19 to 21 (19,20,21)
// //       22 => "XL",
// //       _ => "Not Available",
// //   };
// //   println!("{}", tshirt_size); // L


// //   let is_allowed = false;
// //   let list_type = match is_allowed {
// //       true => "Full",
// //       false => "Restricted"
// //       // no default/ _ condition can be skipped
// //       // Because data type of is_allowed is boolean and all possibilities checked on conditions
// //   };
// //   println!("{}", list_type); // Restricted


// //   let marks_paper_a: u8 = 25;
// //   let marks_paper_b: u8 = 30;
// //   let output = match (marks_paper_a, marks_paper_b) {
// //       (50, 50) => "Full marks for both papers",
// //       (50, _) => "Full marks for paper A",
// //       (_, 50) => "Full marks for paper B",
// //       (x, y) if x > 25 && y > 25 => "Good",
// //       (_, _) => "Work hard"
// //   };
// //   println!("{}", output); // Work hard
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let mut a = 1;
// //   while a <= 10 {
// //       println!("Current value : {}", a);
// //       a += 1; //no ++ or -- on Rust
// //   }

// //   // Usage of break and continue
// //   let mut b = 0;
// //   while b < 5 {
// //       if b == 0 {
// //           println!("Skip value : {}", b);
// //           b += 1;
// //           continue;
// //       } else if b == 2 {
// //           println!("Break At : {}", b);
// //           break;
// //       }
// //       println!("Current value : {}", b);
// //       b += 1;
// //   }

// //   // Outer break
// //   let mut c1 = 1;
// //   'outer_while: while c1 < 6 { //set label outer_while
// //       let mut c2 = 1;
// //       'inner_while: while c2 < 6 {
// //           println!("Current Value : [{}][{}]", c1, c2);
// //           if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
// //           c2 += 1;
// //       }
// //       c1 += 1;
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //     loop {
// //       println!("Loop forever!");
// //   }

// //   // Usage of break and continue
// //   let mut a = 0;
// //   loop {
// //       if a == 0 {
// //           println!("Skip Value : {}", a);
// //           a += 1;
// //           continue;
// //       } else if a == 2 {
// //           println!("Break At : {}", a);
// //           break;
// //       }
// //       println!("Current Value : {}", a);
// //       a += 1;
// //   }

// //   // Outer break
// //   let mut b1 = 1;
// //   'outer_loop: loop { //set label outer_loop
// //     let mut b2 = 1;
// //     'inner_loop: loop {
// //       println!("Current Value : [{}][{}]", b1, b2);
// //       if b1 == 2 && b2 == 2 {
// //           break 'outer_loop; // kill outer_loop
// //       } else if b2 == 5 {
// //           break;
// //       }
// //       b2 += 1;
// //     }
// //     b1 += 1;
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   for a in 0..10 { //(a = o; a <10; a++) // 0 to 10(exclusive)
// //     println!("Current value : {}", a);
// //   }

// //   // Usage of break and continue
// //   for b in 0..6 {
// //     if b == 0 {
// //       println!("Skip Value : {}", b);
// //       continue;
// //     } else if b == 2 {
// //       println!("Break At : {}", b);
// //       break;
// //     }
// //     println!("Current value : {}", b);
// //   }

// //   // Outer break
// //   'outer_for: for c1 in 1..6 { //set label outer_for
// //     'inner_for: for c2 in 1..6 {
// //       println!("Current Value : [{}][{}]", c1, c2);
// //       if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
// //     }
// //   }


// //   // Working with arrays/vectors
// //   let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

// //   for n in 0..group.len() { //group.len() = 4 -> 0..4 👎 check group.len()on each iteration
// //     println!("Current Person : {}", group[n]);
// //   }

// //   for person in group.iter() { //👍 group.iter() turn the array into a simple iterator
// //     println!("Current Person : {}", person);
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let mut a2: Vec<i32> = Vec::new();
// //   let mut b2: Vec<i32> = vec![];
// //   let mut b3 = vec![1i32, 2, 3];//Sufixing 1st value with data type

// //   let mut b4 = vec![1, 2, 3];
// //   let mut b5: Vec<i32> = vec![1, 2, 3];
// //   let mut b6  = vec![1i32, 2, 3];
// //   let mut b7 = vec![0; 10]; //Ten zeroes
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   //Accessing and changing existing data
// //   let mut c = vec![5, 4, 3, 2, 1];
// //   c[0] = 1;
// //   c[1] = 2;
// //   //c[6] = 2; Cannot assign values this way, index out of bounds
// //   println!("{:?}", c); //[1, 2, 3, 2, 1]

// //   //push and pop
// //   let mut d: Vec<i32> = Vec::new();
// //   d.push(1); //[1] : Add an element to the end
// //   d.push(2); //[1, 2]
// //   d.pop(); //[1] : : Remove an element from the end
// //   // 🔎 Capacity and reallocation
// //   let mut e: Vec<i32> = Vec::with_capacity(10);
// //   println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

// //   // These are all done without reallocating...
// //   for i in 0..10 {
// //       e.push(i);
// //   }
// //   // ...but this may make the vector reallocate
// //   e.push(11);
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   let mut v = vec![1, 2, 3, 4, 5];

// //   for i in &v {
// //       println!("A reference to {}", i);
// //   }

// //   for i in &mut v {
// //       println!("A mutable reference to {}", i);
// //   }

// //   for i in v {
// //       println!("Take ownership of the vector and its element {}", i);
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // // Struct Declaration
// // struct Color {
// //     red: u8,
// //     green: u8,
// //     blue: u8
// // }

// // fn main() {
// //   // Creating an instance
// //   let black = Color {red: 0, green: 0, blue: 0};

// //   // Accessing its fields using dot notation
// //   println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); //Black = rgb(0, 0, 0)

// //   // Structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
// //   let mut link_color = Color {red: 0,green: 0,blue: 255};
// //   link_color.blue = 238;
// //   println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)

// //   // Copy elements from another instance
// //   let blue = Color {blue: 255, .. link_color};
// //   println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)

// //   // Destructure the instance using a `let` binding, this will not destruct blue instance
// //   let Color {red: r, green: g, blue: b} = blue;
// //   println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

// //   // Creating an instance via functions & accessing its fields
// //   let midnightblue = get_midnightblue_color();
// //   println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)

// //   // Destructure the instance using a `let` binding
// //   let Color {red: r, green: g, blue: b} = get_midnightblue_color();
// //   println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)
// // }

// // fn get_midnightblue_color() -> Color {
// //     Color {red: 25, green: 25, blue: 112}
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Color(u8, u8, u8);
// // struct Kilometers(i32);

// // fn main() {
// //   // Creating an instance
// //   let black = Color(0, 0, 0);

// //   // Destructure the instance using a `let` binding, this will not destruct black instance
// //   let Color(r, g, b) = black;
// //   println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);

// //   // Newtype pattern
// //   let distance = Kilometers(20);
// //   // Destructure the instance using a `let` binding
// //   let Kilometers(distance_in_km) = distance;
// //   println!("The distance: {} km", distance_in_km); //The distance: 20 km
// // }
// // 
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Electron;

// // fn main() {
// //   let x = Electron;
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   enum Day {
// //     Sunday,
// //     Monday,
// //     Tuesday,
// //     Wednesday,
// //     Thursday,
// //     Friday,
// //     Saturday
// //   }
// // }

// // The `Day` is the enum
// // Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday are the variants
// ////////////////////////////////////////////////////////////////////////////////////
// // enum FlashMessage {
// //   Success, // A unit variant
// //   Warning{ category: i32, message: String }, // A struct variant
// //   Error(String) // A tuple variant
// // }

// // fn main() {
// //   let mut form_status = FlashMessage::Success;
// //   print_flash_message(form_status);

// //   form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
// //   print_flash_message(form_status);

// //   form_status = FlashMessage::Error(String::from("Connection Error"));
// //   print_flash_message(form_status);
// // }

// // fn print_flash_message(m : FlashMessage) {
// //   // Pattern matching with enum
// //   match m {
// //     FlashMessage::Success =>
// //       println!("Form Submitted correctly"),
// //     FlashMessage::Warning {category, message} => // Destructure, should use same field names
// //       println!("Warning : {} - {}", category, message),
// //     FlashMessage::Error(msg) =>
// //       println!("Error : {}", msg)
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   fn takes_anything<T>(x: T) { // x has type T, T is a generic type
// //   }

// //   fn takes_two_of_the_same_things<T>(x: T, y: T) { // Both x and y has the same type
// //   }

// //   fn takes_two_things<T, U>(x: T, y: U) { // Multiple types
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Point<T> {
// //   x: T,
// //   y: T,
// // }

// // fn main() {
// //   let point_a = Point { x: 0, y: 0 }; // T is a int type
// //   let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type
// // }

// // // 🔎 When adding an implementation for a generic struct, the type parameters should be declared after the impl as well
// // //   impl<T> Point<T> {
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   enum Option<T> {
// //     Some(T),
// //     None,
// //  }

// //  enum Result<T, E> {
// //     Ok(T),
// //     Err(E),
// //  }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Player {
// //     first_name: String,
// //     last_name: String,
// // }

// // impl Player {
// //     fn full_name(&self) -> String {
// //         format!("{} {}", self.first_name, self.last_name)
// //     }
// // }

// // fn main() {
// //     let player_1 = Player {
// //         first_name: "Rafael".to_string(),
// //         last_name: "Nadal".to_string(),
// //     };

// //     println!("Player 01: {}", player_1.full_name());
// // }

// // // ⭐️ Implementation must appear in the same crate as the self type

// // // 💡 And also in Rust, new traits can be implemented for existing types even for types like i8, f64 and etc.
// // // Same way existing traits can be implemented for new types you are creating.
// // // But we can not implement existing traits into existing types.
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Player {
// //     first_name: String,
// //     last_name: String,
// // }

// // trait FullName {
// //     fn full_name(&self) -> String;
// // }

// // impl FullName for Player {
// //     fn full_name(&self) -> String {
// //         format!("{} {}", self.first_name, self.last_name)
// //     }
// // }

// // fn main() {
// //     let player_2 = Player {
// //         first_name: "Roger".to_string(),
// //         last_name: "Federer".to_string(),
// //     };

// //     println!("Player 02: {}", player_2.full_name());
// // }

// // // 🔎 Other than functions, traits can contain constants and types.
// ////////////////////////////////////////////////////////////////////////////////////
// // fn main() {
// //   trait Foo {
// //     fn bar(&self);
// //     fn baz(&self) { println!("We called baz."); }
// //   }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // struct Player {
// //     first_name: String,
// //     last_name: String,
// // }

// // impl Player {
// //     fn new(first_name: String, last_name: String) -> Player {
// //         Player {
// //             first_name : first_name,
// //             last_name : last_name,
// //         }
// //     }

// //     fn full_name(&self) -> String {
// //         format!("{} {}", self.first_name, self.last_name)
// //     }
// // }

// // fn main() {
// //     let player_name = Player::new("Serena".to_string(), "Williams".to_string()).full_name();
// //     println!("Player: {}", player_name);
// // }

// // // We have used :: notation for `new()` and . notation for `full_name()`

// // // 🔎 Also in here, instead of using new() and full_name() separately as two expressions, 
// // // we can use Method Chaining. ex. `player.add_points(2).get_point_count();`
// ////////////////////////////////////////////////////////////////////////////////////
// // trait GetSound {
// //     fn get_sound(&self) -> String;
// // }

// // struct Cat {
// //     sound: String,
// // }
// //     impl GetSound for Cat {
// //         fn get_sound(&self) -> String {
// //             self.sound.clone()
// //         }
// //     }

// // struct Bell {
// //     sound: String,
// // }
// //     impl GetSound for Bell {
// //         fn get_sound(&self) -> String {
// //             self.sound.clone()
// //         }
// //     }


// // fn make_sound<T: GetSound>(t: &T) {
// //     println!("{}!", t.get_sound())
// // }

// // fn main() {
// //     let kitty = Cat { sound: "Meow".to_string() };
// //     let the_bell = Bell { sound: "Ding Dong".to_string() };

// //     make_sound(&kitty); // Meow!
// //     make_sound(&the_bell); // Ding Dong!
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // enum Direction {
// //     forward,
// //     backward,
// //     left,
// //     right,
// // }

// // fn main() {
// //     let game_direction:Direction = Direction::forward;
// //         match game_direction {
// //             Direction::forward => println!("Vehicle is moved forward"),
// //             Direction::backward => println!("Car is backward"),
// //             Direction::left => println!("Left to right"),
// //             Direction::right => println!("I turned right"),
// //         }
// // }
// ////////////////////////////////////////////////////////////////////////////////////
// // fn add_one (x :Option<i32>) -> Option<i32> {
// //     match x {
// //         None => None,
// //         Some(i) => Some(i+1),
// //     }
// // }
// // fn main() {
// //     let c = 5;
// //     let p:Option<i32>;
// //     if c == 0 {p = None;}
// //     else { p = Some(c);}
// //     println!("{:?}",add_one(p).unwrap());
// // }
// ////////////////////////////////////////////////////////////////////////////////////
fn main() {
  mod dealy_meal {
     pub mod dinner {
         pub fn dinner() {}
     }
     pub mod lunch {
         pub fn lunch() {}
     }
     pub mod breakfast {
         pub fn breakfast() {}
     }
  }
 }
// /////////////////////////////////////////////////////////////////////////////////////
