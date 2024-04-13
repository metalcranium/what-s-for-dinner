use std::io;
use rand::Rng;

struct DinnerList {
    dinners: Vec<String>,
    dinner: String,
}
impl DinnerList {
    fn add_dinners(&mut self){
        while self.dinner.trim() != "done"{ 
            self.dinner = String::new();
            println!("\nEnter a dinner idea:  ");
            io::stdin().read_line(&mut self.dinner).expect("Unable to read line");
            self.dinners.push(self.dinner.clone());
        }
        self.dinners.remove(self.dinners.len() - 1);
    }
    fn this_dinner(&self){
        let random = rand::thread_rng().gen_range(0..=7);
        if random >= self.dinners.len(){
            self.this_dinner();
        }
        else {
            println!("Tonight's dinner is {}", self.dinners[random]);
        }
    }
}
fn main(){
    let mut dinner = DinnerList{
        dinners: Vec::new(),    
        dinner: String::new(),
    };
    dinner.add_dinners();
    dinner.this_dinner();
}
