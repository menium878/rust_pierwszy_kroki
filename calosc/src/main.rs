#![allow(unused)] //jeżeli chcemy żeby compilator nie krzzyczał o nieużywane zmienne

use std::char::MAX;
use std::f32::consts::PI;
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};

// fn say_hello(){
//     println!("Hello");
// }

// fn get_sum(x:i32,y:i32)-> (i32,i32){
//     println!("{} + {} = {}",x,y,x+y);
//     (x+y,y+x)
// }
// fn sum_vec(vektor:&[i32]) -> i32{
//     let mut sum:i32=0;
//     for i in vektor{
//         sum+=i;
//     }
//     sum
    
// }

/*
If you don't need to add or remove elements from the vector, use slice (&[T]) or mutable slice (&mut [T]):

fn find_factors(n: u64, prime_factors: &mut [u64]) { ... }

let mut prime_factors: Vec<u64> = Vec::new();
find_factors(1134 as u64, prime_factors.as_mut_slice());
Immutable slices allow you to read elements or create subslices; mutable slices additionally allow to modify elements. But slices cannot grow - they are just a view into some vector.

It looks like you need to append new elements to the vector. You won't be able to do it using slice, as I said; you need to pass the vector itself using mutable reference:

fn find_factors(n: u64, prime_factors: &mut Vec<u64>) {
    // here you can call e.g. prime_factors.push(...)
}

let mut prime_factors: Vec<u64> = Vec::new();
find_factors(1134 as u64, &mut prime_factors);

 */
// use std::ops::Add;


// fn get_sum_gen<T:Add<Output = T>>(x: T,y: T) ->T {
//     x+y
// }

// fn print_str(x:&str){
//     println!("{}",x);
// }

// fn print_str_return(x:String) -> String{
//     println!("{}",x);
//     x
// }
// fn change_str(x:&mut String,y:&mut String,){
//     x.push_str("alamakota");
//     y.push_str("string");
//     println!("{}{}",x,y);
//}
mod restauran;
use crate::restauran::order_food;

fn main() {
    //order_food();
    // struct Customer{
    //     name :String,
    //     adress: String,
    //     balance: f32,
    // }    

    // let mut bob =Customer{
    //     name: String::from("Bob"),
    //     adress: String::from("Katowice"),
    //     balance: 32.5,
    // };
    // bob.adress =String::from("Chorzów");
    


    // struct Rectangle<T,U>{
    //     length: T,
    //     height: U,
    // };
    // let rec =Rectangle{
    //     length: 4,
    //     height: 5.3,
    // };
    // trait Shape{
    //     fn new(lenght:f32,width: f32) -> Self;
    //     fn area(&self)->f32;
    // }

    // struct Square{
    //     lenght: f32,
    //     width: f32,
    // };
    // struct Circle{
    //     lenght: f32,
    //     width: f32,
    // };

    // impl Shape for Square{
    //     fn new(lenght:f32,width: f32) -> Square{
    //         Square{lenght,width}
    //     }
    //     fn area(&self)->f32{
    //         self.lenght * self.width
    //     }
    // }
    // impl Shape for Circle{
    //     fn new(lenght:f32,width: f32) -> Circle{
    //         Circle{lenght,width}
    //     }
    //     fn area(&self)->f32{
    //         (self.lenght/2.0).powf(2.0)*PI
    //     }
    // }
    // let mut x = Square{
    //     lenght: 3.2,
    //     width: 1.2,
    // };
    // println!("{}",x.area());

    // let sq: Square=Shape::new(11.2, 12.5); // !traits ważna sprawa
    
    

    // let mut str1=String::from("toot");
    // print_str(&str1); // !muszę skolonować bo inaczej mi umiera w funkcji wartość trochę nie rozumiem albo jak przekaże go jako referencja do stringa to odda  mi go 
    // let a =print_str_return("cos".to_string());
    // println!("{}",a);
    // let mut str2=String::from("cos");
    // change_str(&mut str1,&mut str2);




    // println!("{}",get_sum_gen( 5,4));
    // println!("{}",get_sum_gen( 5.2,4.3));
    // let (a,b)= get_sum(5,4);
    // println!("{} = {}",a,b);

    // let num_vec = vec![1,2,3,4];
    // println!("{}",sum_vec(&num_vec));

/* 
    println!("Jakie jest Twoje imię?");
    let mut imie=String::new();
    let powitanie="Miło Cię poznać";
    io::stdin().read_line(&mut imie)
        .expect("Nie otrzymałem inputu"); //Return Result więc musimy handle if fail

    println!("Hello, {}! {}",imie.trim_end(),powitanie); //trim_end do niezczytywania nowej lini usunięcie białego znaku końcówka

    const zmienna:u32=1_000_000;
    const PI:f32 =3.141592;
    let age="47" //string ""
    let litera='a'; // char ''
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age +1;
    println!("I'm {} and I want ${}",age,zmienna)
 

    //Unsigned int: u8, u16, u32, u64, u128, unsize
    //Signed integer: i8, i16, i32, i64, i128, isize

    println!("Max u32 {}",u32::MAX); // Ile zajmuje
    println!("Max i32 {}",i32::MAX); 

    let _is_true=true;
    
    let num_1:f32 =1.111111111111111;
    println!("f32:{}", num_1+num_1)

    let ranfom_num = rand::thread_rng().gen_range(1..101);
    println!("Rando : {}",ranfom_num);

    
    let age=8;
    if !(1..=18).contains(&age){ //fajny syntax
        println!("Important Bday 1");
    }else if (age ==21) || (age==50){
        println!("Important Bday 2");
    }else if age >65 {
        println!("Important Bday");
    }else{
        println!("Not important Bday");
    }
    let my_age = 47;
    let can_vote=if my_age>=18{
        true
    }else{
        false
    };
    println!("Can Vote: {}",can_vote)
    

    let age2 =8;
    match age2 {
        1..=18 =>println!("Important Bday"),
        21 | 50 => println!("Important Bday"),
        65..=i32::MAX => println!("Important Bday"),
        _ => println!("Nieważne")
    }

    let my_age = 18;
    let voting_age =18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gain the right to vote"),
    };
    */
    //Arrays - tablice must me the same datatype fix size
    // let arr_1 = [1,2,3,4];
    // println!("first {}",arr_1[0]);
    // println!("len {}",arr_1.len());

    // let arr_2= [1,2,3,4,5,6,7,8,9];
    // println!("first {:#?}",arr_2);
    // let mut loop_idx=0;
    // loop {
    //     if arr_2[loop_idx] % 2 ==0{
    //         loop_idx +=1;
    //         continue;
    //     }
    //     if arr_2[loop_idx] == 9{
    //         break;
    //     }
    //     println!("Val : {}",arr_2[loop_idx]);
    //     loop_idx +=1;
    // }

    // for value in arr_2.iter(){
    //     println!("{}",value)
    // }

    // while loop_idx < arr_2.len(){
    //     loop_idx+=1;
    // }
    
    //tuple wiele typów
    // let my_tuple:(u8,String,f64) = (47,"Derek".to_string(),32.3);
    // println!("{:#?}",my_tuple);
    
    // let (v1,v2,v3) = my_tuple;
    // println!("{}",v1);

    // let mut st1 = String::new();
    // st1.push('A'); // push na koniec stringa
    // st1.push_str(" word");
    // for word in st1.split_whitespace(){
    //     println!("{}",word)
    // }
    // let st2 = st1.replace("A", "Another");
    // println!("{}",st2);

    // let st3 = String:: from("x r t b h k k a m c ");
    // let mut v1: Vec<char> = st3.chars().collect(); //vektor to array o nieokreślonej długości ale jako że array to ten sam typ
    // v1.sort(); //sortowanie
    // v1.dedup(); //usuniecie duplikatów
    // for char in v1 {
    //     println!("{}",char);
    // }
    // let st4: &str = "Random string";
    // let mut st5:String = st4.to_string();
    // println!("{}",st5);
    // let byte_arr1 = st5.as_bytes(); //array of bytes
    // let st6 =  &st5[0..6]; //slice of string
    // println!("String length : {}",st6.len());
    // st5.clear(); // remove from string if mutable
    // let st6 = String::from("Just some");
    // let st7 = String::from("words");
    // let st8 =st6 + &st7; //&referencja
    
    // for char in st8.bytes(){
    //     println!("{}",char);
    // }

    // //casting
    // let int_u8:u8=5;
    // let int2_u8:u8=4;
    // let int3_u32: u32 = (int_u8 as u32) +(int2_u8 as u32); // casting as specific type

    // let st_1:String = int_u8.to_string();
    // println!("{}",st_1);

    // enum Day{
    //    Monday,
    //    Tuesday,
    //    Wensday,
    //    Thursday,
    //    Friday,
    //    Saturday,
    //    Sunday
    // }
    // impl Day{
    //     fn is_weekend(&self) ->bool {
    //         match self{
    //             Day::Saturday | Day::Sunday => true,
    //             _  => false,
    //         }
    //     }
    // }
    // let today:Day = Day::Monday;
    // match today{
    //     Day::Monday => println!("Everybody hatest Monday"),
    //     _ => println!("It's not Monday")
    // }
    // println!("{}",Day::is_weekend(&today));

    //Vector the same type like array

    // let vec1:Vec<i32> = Vec::new();
    // let mut vec2 = vec![1,2,3,4];
    // vec2.push(5);
    // println!("{}",vec2[0]);
    // println!("{:#?}",vec2);
    // let secound = &vec2[1];
    // match vec2.get(1){
    //     Some(secound) =>println!("2nd {}",secound),
    //     None => println!("No 2nd value"),
    // }
    
    // for i in &mut vec2{ //to jeśli chcemy iterować po vektorze i zmienić jego wartości referencja do vec czyli &mut vec2
    //     println!("{}",*i);
    //     *i *=2; // dereferencja do wartości i bo odnośmy się do referencji bo i jest &mut i32 w tym przypadku
    //     println!("{}",i);
    // }

    // let mut a =String::from("ab"); 
    // let mut b =&a; // jak przekaże tak to mam tylko referencje nie mam możliwości zminay zawartości
    // //b.push('2'); // czyli to nie działa
    // println!("{:p}",&a); //ten sam adres
    // println!("{:p}",&b); //ten sam adres

    // let mut a =String::from("ab"); 
    // println!("{:p}",&a); 
    // let mut b =&mut a; // daje mu możliwość zmiany wartości
    // b.push('2'); // jak zrobie coś takiego to mam wszystko od a 
    // //println!("{:p}",&a); //to już nie działa oddał w całości do b
    // println!("{:p}",b); 
    // let b =2;
    // println!("{:p}",&b); 
    // println!("{:p}",&a); //oddaliśmy do a bo b przestało istni
    
    // let str1 =String::from("World");
    // let str2 = str1.clone();
    // println!("{}",str1);

    // let mut heros = HashMap::new();
    // heros.insert("Superman", "Clark Kent");
    // heros.insert("Batman", "Bruce Wayne");
    // heros.insert("Flash", "Barry Allen");

    // for (v,k) in heros.iter(){ //!bardzo ważne pamięctaj iter bo inaczej tracisz kontrole nad wartością lul
    //     println!("{}",v);
    //     println!("{}",k);
    // }
    // if heros.contains_key(&"Batman"){
    //     let the_batman=heros.get(&"Batman");
    //     match the_batman {
    //         Some(x)=>println!("{}",x),
    //         None => println!("nie znalazlo"),
    //     }
    // }
    //panic!("Terrible Error");
    
    // let path ="lines.txt";
    // let output =File::create(path);
    // let mut output = match output{
    //     Ok(file)=> file,
    //     Err(error)=>
    //         panic!("Problem creating file : {:?}",error),
    // };
    // write!(output,"Just some\n Random words").expect("Failed to write to the file");

    // let input = File::open(path).unwrap(); // !ignoruje i daje tylko output funkcji
    // let buffered = BufReader::new(input);
    // for line in buffered.lines(){
    //     println!("{}",line.unwrap());
    // }

    // let output2=File::create("rand.txt");
    // let output2 = match output2{
    //     Ok(file)=>file,
    //     Err(error)=>match error.kind(){
    //         ErrorKind::NotFound =>match File::create("rand.txt") {
    //             Ok(fc)=>fc,
    //             Err(e)=>panic!("Can't provide file {:?}",e),
                
    //         },
    //         _other_error => panic!("Problem opening file: {:?}",error),
    //     },
    // };

    // iterator
    // let mut arr_it =[1,2,3,4];
    // for val in arr_it.iter(){
    //     println!("{}",val);
    // }

    // //arr_it.into_iter() // ! pochlaniamy 
    // let mut iter1=arr_it.iter();
    // println!("1st: {:?}",iter1.next());

    // !closuer unnamed funtion czy coś takiego jak lambda
    //let var_name = |parameters| -> return_type{BODY};
    // let can_vote=|age:i32,age_1:i32|{
    //     age>=18
    // };
    // println!("Can vote {}",can_vote(8,9));
    
        // let mut samp1 =5;
        // let print_var = || println!("samp1={}",samp1); // ! możemy używać zmienny z poza funkjci to może być przydatna sprawa
        // print_var();
        // samp1 =10;
        // let mut change_var = ||samp1 +=1;
        // change_var();
        // println!("Samp1 = {}",samp1);
        // fn use_func<T>(a:i32,b:i32,func:T)->i32 
        // where T: Fn(i32,i32)->i32{
        //     func(a,b)
        // }
        // let sum=|a,b|a+b;
        // let prod=|a,b|a*b;
        // println!("5+4= {}",use_func(5, 4, sum));
        // println!("5*4= {}",use_func(5, 4, prod))

    // !smart pointers
    // !&refrence


    // BOX - stores on heep not on stack czyli prawdopodobnie jest szybszy
    // let b_int1 = Box::new(10);
    // println!("b_int1={}",b_int1);
    // struct TreeNode<T>{
    //    pub left:Option<Box<TreeNode<T>>>,
    //    pub right:Option<Box<TreeNode<T>>>,
    //    pub key: T, 
    // }
    // impl<T> TreeNode<T> {
    //     pub fn new(key:T) -> Self{
    //         TreeNode { left: None, right: None, key,}
    //     }
    //     pub fn left(mut self,node:TreeNode<T>)->
    //     Self{
    //         self.left=Some(Box::new(node));
    //         self
    //     }
    //     pub fn right(mut self,node:TreeNode<T>)->
    //     Self{
    //         self.left=Some(Box::new(node));
    //         self
    //     }
    // }
    // let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(5));

    //concurent
    
    // let thread1=thread::spawn(||{
    //     for i in 1..25{
    //         println!("Spawned threead: {}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20{
    //     println!("Main thread :{}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    
    // thread1.join().unwrap();

    pub struct Bank{
        balance: f32
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>,amt:f32){
        let mut bank_ref=the_bank.lock().unwrap();
        if bank_ref.balance<5.00{
            println!("Current Balance {}, Withdrawal a smaller amout",bank_ref.balance);
        }else {
            bank_ref.balance -=amt;
            println!("Customer withdrew{} Current Balance {}",amt,bank_ref.balance);
        }
    }
    fn customer(the_bank:&Arc<Mutex<Bank>>){
        withdraw(the_bank, 5.00);
    }
    let bank=Arc::new(Mutex::new(Bank{balance:20.0}));
    let handles =(0..10).map(|_|{
        let bank_ref=bank.clone();
        thread::spawn(move ||{
            customer(&bank_ref)
        })
    });
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Total {}",bank.lock().unwrap().balance);
}
