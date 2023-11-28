use crate::utils::{cli_input, parse_clipboard};

pub fn insert() {
    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    for d in data {
        println!(
            "INSERT INTO {} (`{}`) VALUES (\"{}\");",
            table_name,
            headers.join("`, `"),
            d.join("\", \""),
        )
    }
}

pub fn update() {
    let fields_input = cli_input("Where Fields:");

    let where_fields: Vec<String> = fields_input
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    for row in data {
        let mut wheres: Vec<String> = vec![];
        let mut sets: Vec<String> = vec![];

        for (index, column) in row.iter().enumerate() {
            let val = format!("`{}`=\"{}\"", &headers[index], column);
            if where_fields
                .iter()
                .filter(|w| w == &&headers[index])
                .count()
                > 0
            {
                wheres.push(val.clone());
            } else {
                sets.push(val.clone());
            }
        }

        print!("UPDATE {} SET ", table_name);

        sets.iter().enumerate().for_each(|(i, s)| {
            print!("{}", s);
            if (i + 1) < sets.len() {
                print!(",");
            }
        });

        wheres.iter().enumerate().for_each(|(i, s)| {
            if i == 0 {
                print!(" WHERE ");
            }
            print!("{}", s);

            if (i + 1) < wheres.len() {
                print!(" AND ");
            }
        });

        println!(";");
    }
}

pub fn insert_bulk() {
    let length = cli_input("Bulk length:").parse::<i32>().unwrap();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    data.chunks(length as usize).for_each(|d| {
        println!(
            "INSERT INTO {} (`{}`) VALUES ",
            table_name,
            headers.join("`, `"),
        );

        for (i, r) in d.iter().enumerate() {
            print!("            (\"{}\")", r.join("\", \""));

            if (i + 1) == d.len() {
                println!(";");
            } else {
                println!(",");
            }
        }
    });
}

fn get_table_name() -> String {
    let table_name = format!("`{}`", cli_input("Table name:"));
    cli_input("Copy data into clipboard and hit ENTER!");

    return table_name;
}
