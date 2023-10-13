use std::ffi::c_void;


fn main(){
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
