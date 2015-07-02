
/*

// example O01

fn main() {

	let x = 15;
	let y = x; // implements i32 Copy 
	println!("{} {}", x, y);



	let v = vec![1, 2, 3];
	let v1 = v;
	//println!("{}", v[0]); //fail because of antiAliasing
	let j = 0;

}

*/


/*

OWNERSHIP TRANSFER.

fn take(v: Vec<i32>)  {
	println!("{}", v[0]); 
	// owner destroy here.
}

fn main() {
	let v = vec![1,2,3];
	take(v);
	//println!("{}", v[0]); //fail because destroyed the owner.
}
*/

/*
// example B01
SHARED BORROWED: inmutable resource and many many borrowers.

fn take(v: &Vec<i32>){
	println!("{}", v[0]); 
	// NOT destroy here. you are not a owner.
}

fn main() {
	let mut v = vec![1,2,3];
	take(&v);
	println!("{}", v[0]);

}
*/



// example B02
//MUTABLE BORROW: one pointer to the resource only one borrower.
/*
fn take(v: &mut Vec<i32>){
	v.push(21);
}

fn main() {
	let mut v = vec![1,2,3];
	//let v = vec![1,2,3]; // fail. why?
	take(&mut v);
	println!("{}", v[3]);

}

*/

/*

fn main() {
	let mut v = vec![1,2,3];

	{
		let x = &mut v;
		let y = &mut v;
		x.push(4);
	}
	println!("{}", v[3]);

}

*/

/*
// example C01
// OWNERSHIP and ACTOR

use std::thread;
use std::sync::mpsc;

fn main() {

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let mut v = vec![1,2,3];
		v.push(21);
		tx.send(v);
	});

	let vect = rx.recv().ok().unwrap();

	println!("{}", vect[3]);

}

*/ 

/*
// example C02
SHARED BORROW OF INMUTABLE.

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = vec![1,2,3];
    let shared_numbers = Arc::new(numbers);

    for id in 0..10 {
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers;
            println!("id: {} vec: {}",id, local_numbers[0]);
        });
    }
}

*/
