pub fn working_with_vectors() {
    // Creating a new vector
    let mut u: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Vector v:{:?}", v);
    // Updating a vector
    u.push(4);
    u.push(5);
    println!("Vector u after push:{:?}", u);
    // Reading vector elements
    let third: &i32 = &v[2]; // reference by indexing
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2); // reference by get
    match third {
        // use match to check if there truly is a third
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // Iterating over values in a vector
    for i in &v {
        // not mutable
        println!("{i}");
    }
    for i in &mut v {
        //  mutable
        *i += 50;
        println!("{i}");
    }
    // Storing multiple types with enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
