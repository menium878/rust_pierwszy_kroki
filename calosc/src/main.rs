//#![allow(unused)] jeżeli chcemy żeby compilator nie krzzyczał o nieużywane zmienne

use std::char::MAX;
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
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
    let arr_1 = [1,2,3,4];
    println!("first {}",arr_1[0]);
    println!("len {}",arr_1.len());

    let arr_2= [1,2,3,4,5,6,7,8,9];
    println!("first {:#?}",arr_2);
    let mut loop_idx=0;
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
    let my_tuple:(u8,String,f64) = (47,"Derek".to_string(),32.3);
    println!("{:#?}",my_tuple);

}
