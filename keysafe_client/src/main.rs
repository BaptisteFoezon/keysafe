use std::io;
use std::fs;

fn main() {
    let mut choice = String::new();
    println!("Bienvenue dans keysafe !!");
    println!("1. Créer un compte");
    println!("2. Se connecter ");
    io::stdin().read_line(&mut choice).expect("Veuillez rentrer une valeur correcte");
    if let choice= String::from("1") {
        let mut nom = String::new();
        let mut prenom = String::new();
        let mut mail = String::new();
        let mut mdp1 = String::new();
        let mut mdp2 = String::new();
        println!("Création de votre compte :");
        println!(". Nom ");
        io::stdin().read_line(&mut nom).expect("Veuillez rentrez une valeur correct");
        //f.write_all(b"Hello, world! ")?; 
        println!(". Prénom ");
        io::stdin().read_line(&mut prenom).expect("Veuillez rentrez une valeur correct");
        println!(". Email ");
        io::stdin().read_line(&mut mail).expect("Veuillez rentrez une valeur correct");
        println!(". mdp ");
        io::stdin().read_line(&mut mdp1).expect("Veuillez rentrez une valeur correct");
        println!(". mdp ");
        io::stdin().read_line(&mut mdp2).expect("Veuillez rentrez une valeur correct");
        if mdp1.to_owned() == mdp2.to_owned() {
            println!("création du compte ....");
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