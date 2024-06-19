fn replace(file_str: String) -> String {
    let mut new_str = String::new();
    
    // look for any reference of "JwIT" and replace it with "NwIT"
    for line in file_str.lines() {
        new_str.push_str(line.replace("JwIT", "NwIT").as_str());
    }

    new_str
}

fn main() {
    // read the file from command flag
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        println!("Please drag vars onto executable.");
        return;
    }

    let file_path = &args[1];
    let file_str = std::fs::read_to_string(file_path).expect("Error reading file");

    // replace the string
    let new_str = replace(file_str);

    // replace the file
    std::fs::write(file_path, new_str).expect("Error writing file");
    
    println!("Done!\nRemove the file named \"LiveDictionary\" manually.");

    // wait for keypress of any
    println!("Press any key to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
