fn recursive_fibonacci(n:i32) -> i32 {
	if n < 0 {
		panic!("{} is negative!", n);
    }
    match n {
		0 => panic!("The Fibonacci sequence has no {}th element!", n),
        1 | 2 => 1,
        _ => recursive_fibonacci(n-1) + recursive_fibonacci(n-2)
    }
}

fn nonrecursive_fibonacci(n: i32) -> i32 {
	if n < 0 {
		panic!("{} is negative!", n);
    }
	else if n == 0 {
		panic!("The Fibonacci sequence has no {}th element!", n);
    }
    let mut a = 0 ;
    let mut b = 1 ;
    for _i in 1..n {
        let temp = a ;
        a = b ;
        b = a + temp ;
    }
    b // Return value
}

fn main() {
    let n = 10 ;
    println!("With recursion, the {}th Fibonacci number is: {}", n, recursive_fibonacci(n));
    println!("Without recursion, the {}th Fibonacci number is: {}", n, nonrecursive_fibonacci(n));
}
