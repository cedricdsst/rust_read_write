use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn read() -> io::Result<String> {
    let file = File::open("mon_fichier.txt")?;

    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn write(texte: &str) -> io::Result<()> {
    let mut file = File::create("mon_fichier.txt")?;

    file.write_all(texte.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let message = "Hello World";

    write(message)?;
    println!("Texte écrit dans le fichier");

    let contenu = read()?;
    println!("Contenu fichier : {}", contenu);

    Ok(())
}
