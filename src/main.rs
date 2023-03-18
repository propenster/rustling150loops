fn main() {
    loop{
        println!("Hello! World.");
        break;
    }

    let mut a = 3;
    while a !=0 {
        println!("{} ", a);
        a = a - 1 ;
    }


    let arr = [0,1,2,3,4,5];
    for value in arr.iter(){
        println!("Value = {} ", value);
    }


}
