use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                vec!["many madrigals".to_string(),
                "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                vec!["The Musicians".to_string(),
                "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                    vec!["Perseus with the head of Medusa".to_string(),
                    "a salt cellar".to_string()]);

    show(&table);
    /*
        after caling show(table), show() takes the ownership
        of table and so table is destroyed;
     */

    assert_eq!(table["Gesualdo"][0], "many madrigals");

    println!("\n\n");

    sort_works(&mut table);

    show(&table);


    let x = 10;
    let r = &x;
    assert!(*r == 10); // explicilty dereference r
    

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    assert!(y == 64);


    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true
    };

    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Aria: The Animation");
    // equivalent to
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    (& mut v).sort();

    {
        let x = 10;
        let y = 20;

        let mut r = &x;
        if true {
            r = &y;
        }

        assert!(*r == 20);
    }

    {
        // references to references
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point {x: 1000, y: 729};
        let r: &Point = &point;
        let rr = &r;
        let rrr = &rr;
        assert_eq!(rrr.y, 729);
        // the `.` operator follows as many references as it takes to find its target
        assert_eq!((***rrr).y, 729);
    }

    {
        // comparing references
        // Rust comparison operators see through any number of references

        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;
        
        let rrx = &rx;
        let rry  = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);
        assert!(!std::ptr::eq(rx, ry));

        // operands of a comparison must have exactly the same type
    }

    {
        /*
            There is no null reference. If you need a value
            that is either a reference or not use the type
            Option<&T>
        */
    }

    {
        // borrowing references to arbitary expressions

        fn factorial(n: usize) -> usize {
            (1..n+1).product()
        }

        let r = &factorial(6);
        assert_eq!(r + &1009, 1729);
    }
}
