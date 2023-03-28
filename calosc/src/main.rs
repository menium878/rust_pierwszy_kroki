#![allow(unused)] //jeżeli chcemy żeby compilator nie krzzyczał o nieużywane zmienne

use std::char::MAX;
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn say_hello(){
    println!("Hello");
}

fn get_sum(x:i32,y:i32)-> (i32,i32){
    println!("{} + {} = {}",x,y,x+y);
    (x+y,y+x)
}
fn sum_vec(vektor:&[i32]) -> i32{
    let mut sum:i32=0;
    for i in vektor{
        sum+=i;
    }
    sum
    
}

fn main() {
    let (a,b)= get_sum(5,4);
    println!("{} = {}",a,b);

    let num_vec = vec![1,2,3,4];
    println!("{}",sum_vec(&num_vec));

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
    


}
