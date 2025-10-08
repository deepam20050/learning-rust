fn main() {
    // {
    //     /*
    //         borrowing a local variable
    //      */
    //     let r;
    //     /*
    //         lifetime of &x must not exceed this range
    //      */
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);
    // }

    {
        let x = 1;
        {
            let  r = &x;
            assert_eq!(*r, 1);
        }
    }

    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1);
        }
    }

    {
        // receiving references as function arguments
       static mut STASH: &i32 = &10;

       fn f(p: &'static i32) {
            unsafe {
                STASH = p;
            }
       }
       // Global variables in Rust have the qualifier 'static 
    }

    {
        // passing references to functions
        fn g<'a>(p: &'a i32) {}

        let x = 10;
        g(&x);
    }

    {
        // returning references

        fn smallest(v: &[i32]) -> i32 {
            let mut s = v[0];
            for r in &v[1..] {
                if *r < s {
                    s = *r;
                }
            }
            s
        }

       let s;
       { 
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
       } 
       assert_eq!(s, 0); // bad: points to element of dropped array 
    }
    
    {
        // returning references

        fn smallest(v: &[i32]) -> &i32 {
            let mut s = &v[0];
            for r in &v[1..] {
                if *r < *s {
                    s = r;
                }
            }
            s
        }

       { 
            let s;
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
            assert_eq!(*s, 0); 
       } 
    }

    {
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        }

        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S { x: &x, y: &y };
                r = s.x;
            }
        }
        println!("{}", r);
    }
}
