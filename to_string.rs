use std::io;

fn main()
{
    let mut string_variable = String::new();

    io::stdin().read_line(&mut string_variable).expect("Failed");
    // The user adds a /n or /r, that's why there is space after printing string_variable below.

    println!("Value before calling anything: {}", string_variable);

    let string_1 = string_variable.trim(); // .trim did not return a reference
    println!("Value after calling string.trim(): {}", string_1);
    // .trim() removes the /n /r character, that's why there is not space after priting the above statement.

    let string_2 = string_variable.trim().to_string();
    println!("Value after calling string.trim().to_string(): {}", string_2);
}

/* 
Conclusion

1. .trim() does not return a reference, but the string itself.
2. After the user gives input, they press enter which stores a /r at the end of the string, that's why the first print statement contains a return after it.
3. .trim() does indeed remove the extra character stored, that's why there is no extra space after the second or third println! statement.
4. Again both:

    * .trim()
    * .trim().to_string()

    Returns the string itself, not a reference, its probably safer to use the .to_string() method anyway.
*/