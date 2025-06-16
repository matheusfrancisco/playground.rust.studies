fn main() {
    let cities = vec!["new", "los angeles", "chicago", "houston", "phoenix"];
    let mut ct = vec![];
    for city in cities.iter() {
        ct.push(city.to_uppercase());
    }
    println!("Cities: {:?}", ct);

    let ct_caps: Vec<String> = cities.into_iter().map(|c| c.to_uppercase()).collect();
    println!("Cities: {:?}", ct_caps);
    let ct_caps = ct_caps.into_iter().map(|c| c.to_uppercase());
    println!("Cities: {:?}", ct_caps);

    let capitalize = |value: &str| value.to_uppercase();
    let cities = vec!["rome", "barcelona", "berlin"];
    let cities_caps: Vec<String> = cities
        .into_iter()
        .filter(|c| c.starts_with("b"))
        .map(capitalize)
        .collect();

    println!("{cities_caps:?}");

    let capitalize = |value: &str| value.to_uppercase();
    let cities = vec!["rome", "barcelona", "berlin"];

    let city_in_caps = cities
        .into_iter()
        .filter(|c| c.starts_with("b"))
        .skip(1)
        .take(1)
        .map(capitalize);

    let cty: Option<String> = city_in_caps.clone().next();

    println!("{cty:?}");

    for (idx, city) in city_in_caps.enumerate() {
        println!("City {idx}: {city}");
    }

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().map(|n| {
        print!("processing {n} -> ");
        n * 2
    });

    println!("number_iter created");
    for n in numbers {
        println!("{n} ");
    }
}
