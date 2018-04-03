use std::slice;

fn main() {
    // Dereferencing a Raw Pointer
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        let address = 0x12345usize;
        let r = address as *const i32;

        println!("r is: {:?}.", r);
        // println!("r1 is: {:?}.", *r1);
        // println!("r2 is: {:?}.", *r2);

        unsafe {
            println!("r is: {:?}.", r);
            println!("r1 is: {:?}.", *r1);
            println!("r2 is: {:?}.", *r2);
        }
    }
    // Calling an Unsafe Function or Method
    {
        unsafe fn dangerous() {
            println!("This is an unsafe function.");
        }
        // dangerous();
        unsafe {
            dangerous();
        }

        // Creating a Safe Abstraction Over Unsafe Code
        {
            #[allow(dead_code)]
            fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
                let len = slice.len();
                let ptr = slice.as_mut_ptr();
                assert!(mid <= len);

                // The safe code below won't compile
                // return (&mut slice[..mid],
                //         &mut slice[mid..]);

                unsafe {
                    return (slice::from_raw_parts_mut(ptr, mid),
                            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid));
                }
            }
            let mut v = vec![1, 2, 3, 4, 5, 6];
            let r = &mut v[..];
            let (a, b) = r.split_at_mut(3);
            println!("{}", a == &mut [1, 2, 3]);
            println!("{}", b == &mut [4, 5, 6]);
        }
        // Using `extern` Functions to Call External Code
        {
            extern "C" {
                fn abs(input: i32) -> i32;
            }
            unsafe {
                println!("Absolute value of -3 according to C: {}", abs(-3));
            }
            
            // Calling Rust Functions from Other Languages
            #[allow(dead_code)]
            #[allow(private_no_mangle_fns)]
            #[no_mangle]
            pub extern "C" fn call_from_c() {
                println!("Just called a Rust function from C!");
            } // This usage of `extern` does not reuqire `unsafe`.
        }
    }
    // Accessing or Modifying a Mutable Static Variable
    {
        static mut COUNTER: u32 = 0;
        // Reading from or writing to a mutable static variable is unsafe.
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}.", COUNTER);
        }
    }
    // Implementing an Unsafe Trait
    {
        // Use `unsafe impl` to implement an unsafe trait
        unsafe trait Foo{
            // Methods go here
        }
        unsafe impl Foo for i32 {
            // Method implementation go here
        }
    }
}
