#[derive(Clone, Copy)]
struct Foo
{
    low: u64,
    high: i64
}

struct FooStruct
{
    bar: Foo
}

impl FooStruct
{
    fn SetBar(&mut self, m: Foo)
    {
        self.bar = m;
    }

    fn GetBar(&self) -> Foo
    {
        self.bar
    }
}

fn main()
{
    let m1 = Foo { low: 1, high: 1 };
    let f = FooStruct { bar: m1 };
    let m2 = Foo { low: 2, high: 2 };
    f.GetBar() = m2;
}
