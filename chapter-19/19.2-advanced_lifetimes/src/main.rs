#![allow(dead_code)]
fn main() {
    // Lifetime Subtyping Ensures One Lifetime Outlives Another
    {
        struct Context<'s>(&'s str);
        struct Parser<'c, 's: 'c> {
            context: &'c Context<'s>,
        }
        impl<'c, 's> Parser<'c, 's> {
            fn parse(&self) -> Result<(), &'s str> {
                return Err(&self.context.0[1..])
            }
        }
        fn parse_context(context: Context) -> Result<(), &str> {
            return Parser { context: &context }.parse();
        }
    }
    // Lifetime Bounds on References to Generic Types
    {
        struct Ref<'a, T: 'a>(&'a T);
        struct StaticRef<T: 'static>(&'static T);
    }
    // Inference of Trait Object Lifetimes
    {
        trait Red { }
        struct Ball<'a> {
            diameter: &'a i32,
        }
        impl<'a> Red for Ball<'a> { }
        let num = 5;
        let _obj = Box::new(Ball { diameter: &num }) as Box<Red>;
    }
}
