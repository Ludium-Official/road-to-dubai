use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    // File Open은 Result 타입을 반환한다

    let f = match f{
        Ok(file)=>file,
        Err(error) if error.kind() == ErrorKind::NotFound =>{
            match File::create("hello.txt") {
                // 
                Ok(fc) => fc,
                Err(e) => {
                    panic!( "Tried to create file but there was a problem: {:?}",
                    e);
                }
            }
        },
        Err(error) => {
            panic!("There is fuckin error :{}", error);
        }


    };
}
