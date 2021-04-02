use std::fs::read_dir;
use std::fs::File;
use std::io::{BufReader, Read, BufRead, Write, Result, stdin, Error};
use std::path::PathBuf;
use structopt::StructOpt;

/** Com esse programar é possivel alterar textos em todos os arquivos contido em um directorio
*** (e até mesmo nos seus subdiretorios)
*** Para invoca-lo, use o formato: <caminho do diretorio> <text a ser alterado> <novo texto>
*** Assim: /home/doc/meus-textos -- biscoiot bolacha
***/

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
fn list_all<'a>(url: &String, files:  &'a mut Vec<PathBuf>) -> Result <&'a mut Vec<PathBuf>>{
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

#[derive(Debug, StructOpt)]
struct Cli {
    path: String,
    old_text: String,
    #[structopt(default_value = "")]
    new_text: String,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    let mut files:Vec<PathBuf> = Vec::new();
    let files = list_all(&cli.path, &mut files).unwrap();

    files.iter().for_each(|path| {
        let content = read_to_string(path).unwrap();
        let new_content =  content.replace(&cli.old_text, &cli.new_text);
        write(path, new_content);
        println!("{}", path.to_str().unwrap());
    });

    Ok(())
}