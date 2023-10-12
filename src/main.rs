use std::io;
fn main() {
    let arr:[i32; 5] = [5, 6, 7, 8, 9];
    //println!("{:#?}", arr.as_ptr());
    unsafe{
    let temp = std::ptr::read((arr.as_ptr() as isize + (std::mem::size_of::<i32>() * 3) as isize) as *const u8);
    println!("{}", temp);
    }
        for i in 1..arr.len(){
        let res = arr.get(i);
        match res{
            Some(value) => println!("{}", value),
            None => {}
        }
    }

    let mut v1:Vec<i32> = vec![66, 67, 68, 69];
    
    v1.push(100);
    let t = v1.iter().map(|x:&i32|(x * 2)).collect::<Vec<i32>>(); 
    //println!("{:#?}", t);
    let t1 =  (1..25).collect::<Vec<i32>>();
    //println!("{:#?}", t1);
    let mut v3: Vec<Vec<i32>> = Vec::new();
    v3.push(v1);
    v3.push(t1);
    //println!("{:#?}", v3);
    
    let name = "laptop";
    for i in name.chars(){
        //println!("{}", i);
    }
    let mut myname = name.to_string();
    myname.push_str(" has popped");
    //println!("{}",myname);
    let u :Vec<u8> = vec![65, 66, 67, 68];
    println!("{}", String::from_utf8_lossy(&u[..]));

    //‐----borrowing

    let mut s = String::from("tech69");
    let reader1 = &s;

    let reader2 = &s;
    //let writer1 = &mut s;
    //testing(&mut s);
    println!("{}", reader1);
    //println!("{}", writer1);
    println!("{}", s);
    //--‐----user input
    user_input();
}

fn testing(name:&mut String){
   println!("{}", name.len());

}
fn user_input(){                                                          let mut input = String::new();                                  println!("Enter count of inputs: ");                            io::stdin().read_line(& mut input).expect("expected an input");                                                                 let count =  input.trim().parse::<i32>().unwrap();
    let mut v1:Vec<i32> = Vec::new();
    for i in 0..count{                                                  let mut temp = String::new();                                   println!("Enter your number: ");                                io::stdin().read_line(& mut temp).expect("expected an input");                                                                  let t1 = temp.trim().parse::<i32>().expect("Integer expected");
        v1.push(t1);                                                }
    println!("All the numbers You've entered: {:?}", v1);       }

