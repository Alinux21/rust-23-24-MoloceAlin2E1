use std::{fs, io};

#[derive(Debug, Clone)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}
fn default_student(_index: usize) -> Student {
    let x: Student = Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    };

    x
}

fn read_students() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/studenti.txt")?;

    let mut students: [Student; 4] = std::array::from_fn(default_student);

    let mut index: usize = 0;
    let mut it = 0;
    for line in s.split('\n') {
        for word in line.split(',') {
            if it % 3 == 0 {
                students[index].name.push_str(word);
            } else if it % 3 == 1 {
                students[index].phone.push_str(word);
            } else {
                for digit in word.chars() {
                    students[index].age = students[index].age * 10 + digit as u8 - '0' as u8;
                }
            }
            it += 1;
        }
        index += 1;
    }

    let mut min_age: u8 = students[0].age;
    let mut max_age: u8 = 0;

    for student in &students {
        if student.age > max_age {
            max_age = student.age;
        }
        if student.age < min_age {
            min_age = student.age;
        }
    }

    println!("Youngest student(s):");
    for student in &students {
        if student.age == min_age {
            println!("{:?}", student);
        }
    }

    println!("\nOldest student(s):");
    for student in students {
        if student.age == max_age {
            println!("{:?}", student);
        }
    }

    Ok(())
}

fn main() {
    let _ = read_students();
}
