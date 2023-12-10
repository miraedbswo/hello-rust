let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
}

if let Some(3) = some_u8_value {
    println!("three");
}

// if let and else
match some_u8_value {
    Some(3) => println!("three"),
    _ => println!("not three"),
}

if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("not three");
}
