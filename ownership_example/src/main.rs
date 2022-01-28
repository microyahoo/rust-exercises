// https://chrismorgan.info/blog/rust-ownership-the-hard-way/
// https://users.rust-lang.org/t/rust-mutability-moving-and-borrowing-the-straight-dope/22166/12
fn main() {
    let age: u8 = 55;
    copyer(age);

    println!("The age is: {}", age);

    println!("===================================================================");
    let mut our_player = Player {
        name: "Jones".to_string(),
        age: 25,
        description: "Just a happy guy.".to_string(),
    };

    immutable_borrow(&our_player);
    change_name(&mut our_player);
    println!(
        "My name is {}, and I am being used after an mutable borrow",
        our_player.name
    );
    our_player.print_me();
    our_player.print_me();
    our_player.change_me();
    our_player.print_me();
    Player::change_me_again(&mut our_player);
    Player::print_me(&our_player);

    println!("===================================================================");
    let mut our_player = Player {
        name: "Tom".to_string(),
        age: 45,
        description: "Just a sad guy.".to_string(),
    };
    let my_immutable_return = immutable_borrow_return(&our_player);
    change_name(&mut our_player);
    println!(
        "My name is {}, and I am being used after an mutable borrow",
        our_player.name
    );
    // my_immutable_return.print_me(); // cannot borrow `our_player` as mutable because it is also borrowed as immutable

    println!("===================================================================");
    let mut a = 6;
    {
        let a_ref = &a;
        take_ref(a_ref);
        // Naturally this will work, for &T is Copy.
        take_ref(a_ref);
    }
    {
        let a_mut = &mut a;
        take_mut(a_mut);
        // Surely a_mut should have been consumed by the line above?
        take_mut(a_mut);
    }
    let x = &mut String::from("xx");
    receive_refmut(x);
    dbg!(x);

    // https://github.com/rust-lang/reference/issues/788
    let mut x = vec![1, 2, 3];
    let y = &mut x;
    let z = &mut *y;
    // let z: &mut Vec<i32> = y;
    // let z = y;
    // z.push(4);
    dbg!(z);
    y.push(5);
    dbg!(y);
    x.push(6);
    dbg!(x);
    // dbg!(y);

    // https://users.rust-lang.org/t/questions-about-mut-t-and-move-semantics-mut-t-is-move-only/37484/15
    // https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html
    // https://exphp.github.io/blog/2018/09/30/lockout-part-3.html
    let mut x: i32 = 5;
    let y = &mut x;

    f(y); // implicit reborrow
    f(&mut *y); // explicit reborrow

    *y += 1;
    println!("{}", *y); // '8'
    x += 1;
    println!("{}", x); // '9'
}

fn f(z: &mut i32) {
    *z += 1;
    println!("{}", *z);
}

// fn f2<'a>(x: &'a mut i32) {
//     let y: &'a mut i32 = x;

//     x; // Does not compile. Cannot move out of `x` because it is borrowed.
// }

// fn no_implicit_reborrow<'a, 'b>(a: &'a mut (), b: &'b mut ()) {
//     fn same_type<T>(_a: T, _b: T) {}

//     same_type(a, b);
//     let _x = a; // error[E0382]: use of moved value: `a`
//     let _y = b; // OK
// }

fn implicit_reborrow<'a, 'b>(a: &'a mut (), b: &'b mut ()) {
    fn same_type<'c>(_a: &'c mut (), _b: &'c mut ()) {}

    same_type(a, b);
    let _x = a; // OK
    let _y = b; // OK
}

fn receive_refmut(y: &mut String) {
    dbg!(&y);
    y.push('*');
}
fn take_ref<T>(_: &T) {}
fn take_mut<T>(_: &mut T) {}

fn copyer(age: u8) -> u8 {
    println!("Age {} has been copied into copyer!", age);
    age
}

struct Player {
    name: String,
    age: u8,
    description: String,
}

impl Player // This just means "define methods on Player"
{
    fn print_me(&self) {
        println!(
            "Name: {}\nAge: {}\nDescription: {}",
            self.name, self.age, self.description
        );
    }

    fn change_me(&mut self) {
        self.name = "changed".to_string();
        self.age = 20;
    }

    fn change_me_again(&mut self) {
        self.name = "changed again".to_string();
        self.age = 200;
    }
}

fn immutable_borrow(borrowed: &Player) {
    println!("I am {}, I've been immutably borrowed", borrowed.name);
}

fn immutable_borrow_return(borrowed: &Player) -> &Player {
    println!("I am {}, I've been immutably borrowed", borrowed.name);
    borrowed
}

fn change_name(borrowed: &mut Player) {
    borrowed.name = "My New Name".to_string();
}
