use std::fs;

fn parse_csv(csv: &str) -> Vec<(String, u32)> {
    let lines = csv.lines();

    let mut csv_data: Vec<(String, u32)> = Vec::new();

    for line in lines {
        let data = line.split(',');

        if data.clone().count() != 2 {
            panic!("Line: \"{}\" has invalid total of items", line);
        }

        let collection: Vec<&str> = data.collect();

        let csv_struct = (
            collection[0].to_string(),
            collection[1].parse::<u32>().unwrap(),
        );

        csv_data.push(csv_struct);
    }

    return csv_data;
}

fn find_struct_size(data: &[(String, u32)], name: &str) -> u32 {
    for elem in data.iter() {
        println!("{}, {}", elem.0, name);
        if elem.0.trim() == name.trim() {
            return elem.1;
        }
    }

    return 0;
}

fn inheritance(input: &str) -> u32 {
    if input.contains(' ') {
        let data = input.split(' ');
        let components: Vec<&str> = data.collect();

        if components[1] == ":" {
            let known_objects: String =
                fs::read_to_string("object_data.csv").expect("Unable to read object_data.csv");
            let struct_data: Vec<(String, u32)> = parse_csv(&known_objects);

            return find_struct_size(&struct_data, components[2]);
        }
    }
    return 0;
}

fn make_struct(name: &str, size: u32) -> String
{
    let mut response = String::new().to_owned();


    response.push_str(&format!("struct {} {}\n", name, "{"));

    let mut items = size;
    let mut prefix = "_BYTE";

    if size % 4 == 0
    {
        prefix = "_DWORD";
        items = (size / 4) as u32;
    }

    for x in 0..items {
        response.push_str(&format!("    {} item_{}\n", prefix, x));
    }

    response.push_str("}");

    return response;
}


fn main() {
    let mut input = String::new();
    let mut size = String::new();

    println!("What is your struct called?\n");

    std::io::stdin().read_line(&mut input).unwrap();

    let inheritance_offset = inheritance(&input);

    println!("How big is your struct? (Base 10)");

    std::io::stdin().read_line(&mut size).unwrap();

    let final_size = size.trim().parse::<u32>().unwrap() - inheritance_offset;

    let res = make_struct(&input, final_size);

    println!("{}", res);
}
