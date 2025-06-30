use std::io;
use std::io::Write;

struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn ajouter_livre(bibli: &mut Vec<Livre>) {
    let mut titre = String::new();
    let mut auteur = String::new();
    let mut annee = String::new();

    print!("Titre: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim().to_string();

    if bibli.iter().any(|l| l.titre == titre) {
        println!("Livre déjà existant.");
        return;
    }

    print!("Auteur: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut auteur).unwrap();
    let auteur = auteur.trim().to_string();

    print!("Année: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut annee).unwrap();
    let annee: u32 = match annee.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Année invalide.");
            return;
        }
    };

    let livre = Livre {
        titre,
        auteur,
        annee,
        disponible: true,
    };

    bibli.push(livre);
    println!("Livre ajouté.");
}

fn afficher_livres(bibli: &Vec<Livre>) {
    for livre in bibli {
        println!(
            "{} - {} ({}) [{}]",
            livre.titre,
            livre.auteur,
            livre.annee,
            if livre.disponible { "dispo" } else { "emprunté" }
        );
    }
}

fn afficher_disponibles(bibli: &Vec<Livre>) {
    for livre in bibli.iter().filter(|l| l.disponible) {
        println!("{} - {} ({})", livre.titre, livre.auteur, livre.annee);
    }
}

fn emprunter_livre(bibli: &mut Vec<Livre>) {
    let mut titre = String::new();
    print!("Titre à emprunter: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim();

    for livre in bibli.iter_mut() {
        if livre.titre == titre {
            if livre.disponible {
                livre.disponible = false;
                println!("Livre emprunté.");
            } else {
                println!("Déjà emprunté.");
            }
            return;
        }
    }

    println!("Livre non trouvé.");
}

fn retourner_livre(bibli: &mut Vec<Livre>) {
    let mut titre = String::new();
    print!("Titre à retourner: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim();

    for livre in bibli.iter_mut() {
        if livre.titre == titre {
            if !livre.disponible {
                livre.disponible = true;
                println!("Livre retourné.");
            } else {
                println!("Ce livre n'était pas emprunté.");
            }
            return;
        }
    }

    println!("Livre non trouvé.");
}

fn main() {
    let mut bibli: Vec<Livre> = Vec::new();

    loop {
        println!();
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles");
        println!("6. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => ajouter_livre(&mut bibli),
            "2" => emprunter_livre(&mut bibli),
            "3" => retourner_livre(&mut bibli),
            "4" => afficher_livres(&bibli),
            "5" => afficher_disponibles(&bibli),
            "6" => break,
            _ => println!("Choix invalide."),
        }
    }
}
