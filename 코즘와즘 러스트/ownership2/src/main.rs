fn main()
{
    let s1 = give_ownership(); // returns value's owenship to s1

    let s2  = String::from("hello");

    let s3 = takes_and_giveback(s2);


}


fn give_ownership() -> String  {
    let some = String::from("your string");
    some // ownership moves by return value.
}


fn takes_and_giveback(string1:String) -> String{
    let tmp = string1;
    tmp
}