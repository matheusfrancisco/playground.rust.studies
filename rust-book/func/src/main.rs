fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_1<T: Fn(i32) -> i32>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_2<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn returns_closure(a: i32) -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_closuree(a: i32) -> Box<dyn Fn(i32) -> i32> {
  if a > 0 {
    Box::new(move |b| a + b)
  } else {
    Box::new(move |b| a - b)
  }
}

fn main() {
    let ans = do_twice(add_one, 5);
    println!("{}", ans);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
