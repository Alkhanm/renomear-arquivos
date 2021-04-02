use clap::{Arg, App};
use std::fs::{read_dir, DirEntry};
use std::fs::File;
use std::io::{BufReader, Read, BufRead, Write, Result, stdin, Error};
use std::path::PathBuf;

fn read_to_string(path: &PathBuf) -> Result<String> {
    let mut contents = String::new();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents);
    return Ok(contents);
}
fn write(path: &PathBuf, content: String) -> Result<()>{
    let mut file = match File::create(path) {
        Ok(content) => content,
        Err(err) => panic!("Ops, houve ao criar um reescrever o arquivo: {}", err)
    };
    file.write_all(content.as_ref());
    Ok(())
}
fn list_all<'a>(url: &str, files:  &'a mut Vec<PathBuf>) -> Result <&'a mut Vec<PathBuf>>{
    read_dir(url)?.for_each(|v| {
        let path = v.unwrap().path();
        if path.is_dir() {
            list_all(&String::from(path.to_str().unwrap()), files);
        } else {
            files.push(path);
        }
    });
    Ok(files)
}

fn main() -> Result<()> {
     let matches = App::new("renom")
        .version("0.1.0")
        .author("Joás <joas2017sousa@hotmail.com>")
        .about("Renomeie substitua textos ou frases contidas em qualquer diretorio ou em seus subdiretorios")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .takes_value(true)
            .help("Caminho para o diretorio. Todos os arquivos contidos a apartir daqui serão afetados."))
        .arg(Arg::with_name("old_text")
            .short("o")
            .long("old")
            .takes_value(true)
            .help("Texto que será substituido"))
        .arg(Arg::with_name("new_text")
            .short("n")
            .long("new")
            .takes_value(true)
            .help("Texto que irá substituir o antigo")
            )
        .get_matches();

    let dir = matches.value_of("path")
        .expect("Erro ao ler o caminho do diretorio"); //.unwrap_or("input.txt");
    let old_text = matches.value_of("old_text")
        .expect("É necessario que haja algum padrão de texto a ser buscado (-- -p '/dir/outro/' -o 'text a ser encontrado')");
    let new_text = matches.value_of("new_text")
        .unwrap_or("");

    let mut files:Vec<PathBuf> = Vec::new();
    let files = list_all(&dir, &mut files).unwrap();

  files.iter().for_each(|path| {
        let content = read_to_string(path).unwrap();
        let new_content =  content.replace(&old_text, &new_text);
        write(path, new_content);
        println!("{}", path.to_str().unwrap());
  });


    Ok(())
}