//P1
/*


fn add_chars_n(mut s: String, c: char, x: u8) -> String {
    let mut cnt = 0;
    while cnt < x {
        s.push(c);
        cnt += 1;
    }

    s
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
*/



//P2
/*
fn add_chars_n(s: &mut String, c: char, x: u8) {
    let mut cnt = 0;
    while cnt < x {
        s.push(c);
        cnt += 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    let ref_to_s: &mut String = &mut s;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(ref_to_s, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
}
*/
//P3
fn add_space(mut s: String, n: i32) -> String {
    let mut cnt = 0i32;
    while cnt < n {
        s.push(' ');
        cnt += 1;
    }
    s
}

fn add_str(mut s: String, r: &str) -> String {
    s.push_str(r);

    s
}

fn add_integer(mut s: String, mut n: i32, delimiter: char) -> String {
    let mut cnt: i32 = 0;
    let mut ogl: i32 = 0;
    while n > 0 {
        ogl = ogl * 10 + n % 10;
        n /= 10;
        cnt += 1;
    }

    while ogl > 0 {
        let c: char = ((ogl % 10) as u8 + 48) as char;
        ogl /= 10;

        s.push(c);

        if cnt % 3 == 1 && cnt != 1 {
            s.push(delimiter);
        }
        cnt -= 1;
    }

    s
}



fn main() {
    let mut s = String::from("");
    s=add_space(s,41);
    s=add_str(s,"I");
    s=add_space(s,1);
    s=add_str(s,"💚\n");


    s=add_space(s,41);
    s=add_str(s,"RUST.\n");
    
    s=add_str(s,"\n");
    s=add_space(s,4);
    s=add_str(s,"Most");
    s=add_space(s,12);
    s=add_str(s,"crate");
    s=add_space(s,6);
    s=add_integer(s,306437968,'_');
    s=add_space(s,11);
    s=add_str(s,"and");
    s=add_space(s,5);
    s=add_str(s,"latest");
    s=add_space(s,10);
    s=add_str(s,"is\n");


    s=add_space(s,9);
    s=add_str(s,"downloaded");
    s=add_space(s,8);
    s=add_str(s,"has");
    s=add_space(s,13);
    s=add_str(s,"downloads");
    s=add_space(s,5);
    s=add_str(s,"the");
    s=add_space(s,9);
    s=add_str(s,"version");
    s=add_space(s,4);
    s=add_str(s,"2.038.\n");
/* 
                                         I 💚
                                         RUST.

    Most            crate      306_437_968           and     lastest         is
         downloaded        has             downloads     the         version    2.038.
                    
       
*/








   

    println!("{s}");

    
}
