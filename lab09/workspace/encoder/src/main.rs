#[cfg(target_os="windows")]
fn get_os_name()->&'static str{
    "Windows"
} 
#[cfg(target_os="linux")]
fn get_os_name()->&'static str{
    "Linux"
} 
#[cfg(target_os="macos")]
fn get_os_name()->&'static str{
    "Mac"
} 

extern crate base64;

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "args parsing example")]
struct Args {
   /// Input file
   #[arg( long, default_value = "stdin")]
   input: String,

   /// Output file
   #[arg( long, default_value = "stdout")]
   output: String,

}

use std::fs;
use std::io;
use std::io::Write;

fn f(input_file:&str,output_file:&str) -> Result<(), io::Error> {
    
    if input_file=="stdin"{  
        let mut buffer = String::new();
        
        print!("Input the text you want to encode:");
        std::io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).expect("Error while reading from stdin");
        buffer.pop();
        let bytes:&[u8]=buffer.as_bytes();
        let encoded_content = base64::encode(bytes);
        println!("Encoding: {}", encoded_content);
    }
           
    else{   //from file
    
    let dir = String::from("src/");
    let input_path = dir + input_file;
    let file_content = fs::read(&input_path)?;
    
    let encoded_content = base64::encode(&file_content);    
    let output_dir=String::from("src/");
    let output_path = output_dir + output_file;
    fs::write(output_path, encoded_content).expect("Error while writing the output file!");

    }

    Ok(())
}

fn main(){


    let version = env!("CARGO_PKG_VERSION");
    println!("encoder, version : {}, built for {}",version,get_os_name());

    let args = Args::parse();

    f(&args.input,&args.output).expect("Error while trying to exec function!");




}