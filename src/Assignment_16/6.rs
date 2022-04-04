// Example:

fn main() {
  let age = 23; 
  let play = true; 
  let activity="Baseball" ;

  if age >= 21 && play==true || activity == "Tennis" {  
      // Here we are first comparing the age with 21 and then we are comparing the play variable with true.And then we are comparing the activity variable with Tennis.
    println!("Age is greater than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else if  age >= 21 && play == true && activity == "Tennis"{ 
            
    println!("Age is greater than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else if age <21 && play == false && activity == "Tennis"{
    println!("Age is less than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else{
    println!("Value Printed");
  }
 }

 // Output: Age is greater than 21
    // You are allowed to play
    // The sport is Baseball