fn main() {
    let weight: usize =400;
    println!("Weight : {}",weight);
}

fn main() {
    let eid =("Namaz","Qurbani",2000,"Seekh Kahab",50);
    println!("{} {} {} {} {}",eid.0,eid.1,eid.2,eid.3,eid.4);
    println!("{:?} ",eid);
}


fn main (){
    let eidiname = ["Sidra","Sajid","Ali"];
    let eidiamount   = [500,400,300];
    println!("{} {}",eidiname[0],eidiamount[0]);
    let a=10;
    println!("{} {}",eidiname[a],eidiamount[a]);
}

fn main (){
   println!("Bakra Eid Mubarak");
   qurbani();
}

fn qurbani(){
    println!("Qurbani");
    print!("Seekh Kabab ");
    print!("Behari Booti  ");
    print!("Nehari  ");
    print!("Mutton Kunna");
}












