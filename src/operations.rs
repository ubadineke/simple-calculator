pub struct Inputs{
  pub number1: i32,
  pub number2: i32
}

impl Inputs{
  pub fn add(&self) -> Option<i32>{
    self.number1.checked_add(self.number2)
  }

  pub fn subtract(&self) -> Option<i32>{
    self.number1.checked_sub(self.number2)
  }

  pub fn multiply(&self) -> Option<i32>{
    self.number1.checked_mul(self.number2)
    // Some(self.number1 * self.number2)
  }

  pub fn divide(&self) -> Option<i32>{
    if self.number2 != 0 {
      self.number1.checked_div(self.number2)
    }else{
      println!("Error: Cannot divide by zero");
      None
    }
  }
}