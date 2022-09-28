use std::io;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::str;

fn main() {
    let mut choice = String::new();
    println!("Bienvenue dans keysafe !!");
    println!("1. Créer un compte");
    println!("2. Se connecter ");
    io::stdin().read_line(&mut choice).expect("Veuillez rentrer une valeur correcte");
    if let choice= String::from("1") {
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte :");
        io::stdin().read_line(&mut mail).expect("Veuillez rentrez une valeur correcte");
        println!(". Password :  ");
        io::stdin().read_line(&mut mdp1).expect("Veuillez rentrez une valeur correcte");
        println!(". Tape again : ");
        io::stdin().read_line(&mut mdp2).expect("Veuillez rentrez une valeur correcte");
        if mdp1.to_owned() == mdp2.to_owned() {
            println!("Création du compte ....");
            users_store(&mail, &mdp2);
            let user = User::new(nom, prenom, mail, mdp2);
            println!("Bienvenue {}", user.get_name())

        }else {
            println!("les mots de passe sont diférents");
        }
    }
    if let choice= String::from("2") {
        println!("choix numéro 1");
    }

}

struct User{
    nom : String,
    prenom : String,
    email : String,
    mdp : String,
}

impl User{
    fn new(nom : String, prenom : String, email : String, mdp : String)-> User{
        User{
            nom,
            prenom,
            email,
            mdp
        }
    }
    fn get_name(&self) -> &str { &self.nom }
    fn get_mail(&self) -> &str{ &self.email }
}

fn users_store(id: &String, mdp: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open("users.txt").expect("Unable to open file");
    //let String id_txt = id;
    println!("Voici le pointeur : {} .", &id);
    file.write_all(id.as_bytes()).expect("Echec d'écriture");
    //file.write_all(nom<).expect("Echec d'écriture");
    Ok(())
}