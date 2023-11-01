use std::{fs, io};

fn p4() -> Result<(), io::Error> {
    let hosts_file = fs::read_to_string("/etc/hosts")?;
    let mut result = String::new();

    let mut column1 = String::new();
    let mut column2 = String::new();

    for line in hosts_file.split("\n") {
        if !line.starts_with('#') {
            for (index, column_hosts) in line.split_whitespace().enumerate() {
                if index == line.split_whitespace().count() - 1 {
                    column2.push_str(column_hosts);
                } else {
                    column1.push_str(column_hosts);
                }
            }
            if !line.trim().is_empty() {
                result.push_str(&column1);
                result.push_str(" => ");
                result.push_str(&column2);
                result.push('\n');
                column1 = String::new();
                column2 = String::new();
            }
        }
    }

    println!("{}", result);

    Ok(())
}

fn main() {
    let _ = p4();
}
