


pub struct Primary {
  pub x: i32, 
  pub y: String,
  pub z: i8
}

impl Primary{
  pub fn test(&self){
    println!("The value of the variables in Primary are :");
    println!("x = {}, y = {}, z = {}", self.x, self.y, self.z);
  }
}



