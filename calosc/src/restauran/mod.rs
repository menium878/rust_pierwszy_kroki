mod pizza_order{
    pub struct Pizza{
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza{
        pub fn lunch(topping: &str)-> Pizza{
            Pizza{
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer{
        fn seat_at_table(){
            println!("Customer seated at tab");
        }
        pub fn take_order(){ //!SUPER is used to acces parent 
            seat_at_table();
            let cust_pizza: super::Pizza=
                supper::Pizza::lunch("veggies");
                serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza:super::Pizza){
            println!("The customer is served a regular pizza with {}",cust_pizza.topping);
        }
    }
}

pub fn order_food(){
    create::restaurant::pizza_order::help_customer::take_order();
}