pub fn insert(table_name: String, headers: Vec<String>, data: Vec<Vec<String>>) {
    let join_headers = headers.join("`, `");

    for d in data {
        println!(
            "INSERT INTO {} (`{}`) VALUES (\"{}\");",
            table_name,
            join_headers,
            d.join("\", \""),
        );
    }
}

pub fn update(
    table_name: String,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    where_fields: Vec<String>,
) {
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

pub fn bulk_insert(
    table_name: String,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    bulk_length: u16,
) {
    let join_headers = headers.join("`, `");

    data.chunks(bulk_length as usize).for_each(|d| {
        println!("INSERT INTO {} (`{}`) VALUES ", table_name, join_headers,);

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

pub fn delete(
    table_name: String,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    where_fields: Vec<String>,
) {
    for row in data {
        let mut wheres: Vec<String> = vec![];

        for (index, column) in row.iter().enumerate() {
            let val = format!("`{}`=\"{}\"", &headers[index], column);
            if where_fields
                .iter()
                .filter(|w| w == &&headers[index])
                .count()
                > 0
            {
                wheres.push(val.clone());
            }
        }

        print!("DELETE FROM {} WHERE", table_name);

        wheres.iter().enumerate().for_each(|(i, s)| {
            print!("{}", s);

            if (i + 1) < wheres.len() {
                print!(" AND ");
            }
        });

        println!(";");
    }
}
