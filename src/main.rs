mod shape;
// use shape::shapes;
fn main(){
 let t = shape::shapes::shape_print("Triangle".to_string(), 3 ,3,0) ;
 println!("{:#?}",t);

 let sq = shape::shapes::shape_print("Square".to_string(), 4 ,4,2) ;
 println!("{:#?}",sq);
 
 let tr = shape::shapes::shape_print("Trapezium".to_string(), 4,4,2);
 println!("{:#?}",tr);

 let tri = shape::shapes::ShapeProp{
     name: "Triangle".to_string(),
     sides: 3,
     angles: 3,
     diagonals: 0,
 };
 println!("{}",tri.build());
}


// mod shape;
// use shape::shapes;
// fn main() {
//     let triangle = shape::shapes::shape_print(
//         "t".to_string(),
//           3,
//     );
//     
// }
