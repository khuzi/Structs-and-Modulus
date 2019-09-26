pub mod shapes{
    #[derive(Debug)]
    pub struct ShapeProp {
       pub name: String,
        pub sides: u8,
        pub diagonals: u8,
        pub angles: u8,
    }
   impl ShapeProp{
         pub  fn build(&self)->String{
                let shap = format!("Shape Name:{}\nShape Sides:{}\nShape Angles:{}\nShape Diagonals:{}",
            self.name,self.sides,self.angles,self.diagonals);
            shap 
           }
   } 





   pub fn shape_print(name: String , sides: u8 , angles: u8,diagonals: u8)->ShapeProp{
        ShapeProp{
            name,
            sides,
            angles,
            diagonals,
        }
    }

       }