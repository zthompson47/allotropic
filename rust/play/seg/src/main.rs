fn main() {
    //println!("min: {}", f32::MIN);
    //println!("max: {}", f32::MAX);
    //println!("epsilon: {}", f32::EPSILON);

    //println!("1: {}", 1.);
    //println!("1 + epsilon: {}", 1. + f32::EPSILON);
    //println!("max + epsilon: {}", f32::MAX + f32::EPSILON);

    //println!("{:#b}", 1.1);

    let a = 1.2 - 0.1;
    let b = 1.0 + 0.1;

    let aa = 3.2 - 0.1;
    let bb = 3.0 + 0.1;

    println!("0.1           {:.55}", 0.1);
    println!("1.2           {:.55}", 1.2);
    println!("1.0           {:.55}", 1.0);
    println!("a: 1.2 - 0.1  {:.55}", a);
    println!("b: 1.0 + 0.1  {:.55}", b);
    println!("b - a         {:.55}", b - a);
    println!("epsilon       {:.55}", f64::EPSILON);
    println!("----------------------------");
    println!("0.1           {:.55}", 0.1);
    println!("3.3           {:.55}", 3.3);
    println!("3.1           {:.55}", 3.1);
    println!("a: 3.3 - 0.1  {:.55}", aa);
    println!("b: 3.1 + 0.1  {:.55}", bb);
    println!("b - a         {:.55}", bb - aa);
    println!("epsilon       {:.55}", f64::EPSILON);
    println!("----------------------------");
    println!("3.0           {:.55}", 3.0);
}
