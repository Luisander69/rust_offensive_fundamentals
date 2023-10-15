
use std::ffi::c_void;
use std::env;
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
    //user_input();
    //-‐--------line arguments
    //command_line_args();
    //--‐--------refferences
    refferences();
}

fn testing(name:&mut String){
   println!("{}", name.len());

}
fn user_input(){                                                          let mut input = String::new();                                  println!("Enter count of inputs: ");                            io::stdin().read_line(& mut input).expect("expected an input");                                                                 let count =  input.trim().parse::<i32>().unwrap();
    let mut v1:Vec<i32> = Vec::new();
    for i in 0..count{                                                  let mut temp = String::new();                                   println!("Enter your number: ");                                io::stdin().read_line(& mut temp).expect("expected an input");                                                                  let t1 = temp.trim().parse::<i32>().expect("Integer expected");
        v1.push(t1);                                                }
    println!("All the numbers You've entered: {:?}", v1);       }

fn command_line_args(){
    let mut args = env::args().collect::<Vec<String>>();
    if args.len()!= 3{
        println!("[+]Usage: app arg1 arg2 \n\n");
        std::process::exit(0);
    }

    println!("{:?}", args);
}

fn refferences(){
    let mut a = 4; 
    let mut b:[i32;3] = [3, 6, 8];
    unsafe{
        //----------variables refference
        let p = &mut a as *mut i32;

        println!("\nVariable refference area|________");
        println!("pointer value: {:x?}", p);
        println!("addres of value: {:x?}", std::ptr::addr_of!(p));
    
        *p = *p + 10;
        let temp = std::ptr::read(p as *const i8);
        std::ptr::write(p, 78);
        println!("Pointer to value changed 2 times {:?}", p);
        println!("Read of p {}", temp);
        //‐-----------array refference

        let p2 = b.as_mut_ptr(); // &mut b as *mut i32
        let temp2 = std::ptr::read((p2 as usize + 4) as *mut i32);
       
        println!("\n\n\nArray refference area|_________");
        println!("Array incrementation {}", temp2);

        let q = b.as_mut_ptr() as *mut c_void;// raw refference in windows
        println!("Raw refference {:?}", q); 

       //------------vector refference
        println!("\n\n\nVector refference area|_________");
        vector_refference();
        println!("\n\n\nEnd");

    }
    println!("End value of a {}", a);
}

fn vector_refference(){

    let mut v:Vec<i32> = vec![5, 6, 7, 8];


    unsafe{
        let p3: *const Vec<i32>  = &v as *const Vec<i32>;
        println!("addres of vector: {:x?}", std::ptr::addr_of!(v));
        let q = v.as_mut_ptr();
        println!("value of q which uses as_mut_ptr: {:x?}", q);
        println!("value of p3 which is const ref {:x?}", p3);
        println!("Adress enum starts:\n\n\n");
        for i in (-256..257).step_by(8){
            let temp3 = std::ptr::read((p3 as isize - i) as *mut i32);
            println!("Step is {}", i);
            println!("Functional read of p {:x?}\n", temp3);
         }
    }
}
//little comment for git testing
