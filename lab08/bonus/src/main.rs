use serde_derive::Deserialize;
use std::io;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Spell {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Spells {
    count: u32,
    results: Vec<Spell>,
}

#[derive(Debug, Deserialize)]
struct FoundSpell{
    name:String,
    desc: Vec<String>,
    level:u32,

}

fn main() -> Result<(), ureq::Error> {
    print!("Search spells by keyword in name:");
    std::io::stdout().flush().unwrap();
    //reading the spell from the user
    let mut user_desired_spell = String::new();
    io::stdin()
        .read_line(&mut user_desired_spell)
        .expect("Error while reading from the user!");
    user_desired_spell.pop();
    // Make a request to the API
    let body: String = ureq::get("https://www.dnd5eapi.co/api/spells")
        .call()?
        .into_string()?;

    // Deserialize the JSON response into a Spells struct
    let spells: Spells = serde_json::from_str(&body).expect("Error while deserializing");


    let mut spells_found:Vec<Spell>=Vec::new();


    for spell in spells.results{

        if spell.name.to_lowercase().contains(&user_desired_spell.to_lowercase()){
            spells_found.push(spell);
        }
    }

    println!("From a total of {:?} spells, {} spells have been found!",spells.count,spells_found.len());

    let mut i=0;
    for spell in spells_found{

            i+=1;
            let domain=String::from("https://www.dnd5eapi.co");
            let search_url=domain+&spell.url.to_string();


            let spell_body: String = ureq::get(&search_url)
        .call()?
        .into_string()?;
            let user_spell: FoundSpell= serde_json::from_str(&spell_body).expect("Error while deserializing");
            println!("{i}. Name: {:?}",user_spell.name);
            println!("Level: {:?}",user_spell.level);
            println!("Description: {:?}\n",user_spell.desc[0]);
       
    }

    Ok(())
}
