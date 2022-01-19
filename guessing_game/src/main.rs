use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::mem::{align_of, size_of};

// 默认表形，把对齐量缩小到2。
#[repr(packed(2))]
struct PackedStruct {
    first: i16,
    second: i8,
    third: i32,
}

// C表形，把对齐量增大到8
#[repr(C, align(8))]
struct AlignedStruct {
    first: i16,
    second: i8,
    third: i32,
}

#[repr(C)]
union SizeRoundedUp {
    a: u32,
    b: [u16; 5],
}

#[repr(C)]
union Union {
    f1: u16,
    f2: [u8; 4],
}

fn main() {
    println!("size_of(PackedStruct) = {}", size_of::<PackedStruct>());
    println!("align_of(PackedStruct) = {}", align_of::<PackedStruct>());

    println!("size_of(AlignedStruct) = {}", size_of::<AlignedStruct>());
    println!("align_of(AlignedStruct) = {}", align_of::<AlignedStruct>());

    println!("size_of(Union) = {}", std::mem::size_of::<Union>());
    println!("align_of(Union) = {}", std::mem::align_of::<Union>()); // 来自于 a

    println!(
        "size_of(SizeRoundedUp) = {}",
        std::mem::size_of::<SizeRoundedUp>()
    );
    println!(
        "align_of(SizeRoundedUp) = {}",
        std::mem::align_of::<SizeRoundedUp>()
    ); // 来自于 a

    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your number:");

        // let foo = 1;
        // let bar = foo;
        // foo = 3; // compiler error

        let mut guess = String::new(); // associated function

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
