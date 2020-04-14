// // fn main(){
// //     let mut vector_integer: Vec<i32> = vec![20,30];
// //     vector_integer.push(40);
// //     println!("{:?}",vector_integer);
// //    }




//              /////this program willl not run
// // fn main(){
// //     let mut vector_integer: Vec<i32> = vec![20,30];
// //     vector_integer.push(40);
// //     vector_integer.push("hello"); //error[E0308]: mismatched types
// //     println!("{:?}",vector_integer);
// //    }





// struct Data<T> {
//     value:T,
//     }
   
//    fn main(){
//            ////generic type of i32
//     let t:Data<i32> = Data{value:350};
//     println!("value is :{} ",t.value);
//           ////generic type of String
//     let t2:Data<String> = Data{value:"Tom".to_string()};
//     println!("value is :{} ",t2.value);
//    }





fn main(){
    //create an instance of the structure
     let b1 = Book {
     id:1001,
     name:"Rust in Action"
     };
     b1.print();
    }
    //declare a structure
    struct Book {
     name:&'static str,
     id:u32
    }
    //declare a trait
    trait Printable {
     fn print(&self);
    }
    //implement the trait
    impl Printable for Book {
     fn print(&self){
     println!("Printing book with id:{} and name {}",self.id,self.name)
     }
    }