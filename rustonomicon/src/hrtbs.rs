pub struct Closure<F> {
    pub data: (u8, u16),
    pub func: F,
}

impl<F> Closure<F>
where
    for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    pub fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

pub fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}
