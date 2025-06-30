use std::io;

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

    println!("Titre ?");
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim().to_string();

    for l in bibli.iter() {
        if l.titre == titre {
            println!("Titre déjà utilisé.");
            return;
        }
    }

    println!("Auteur ?");
    io::stdin().read_line(&mut auteur).unwrap();

    println!("Année ?");
    io::stdin().read_line(&mut annee).unwrap();
    let annee: u32 = annee.trim().parse().unwrap_or(0);

    let livre = Livre {
        titre,
        auteur: auteur, 
        annee,
        disponible: true,
    };

    bibli.push(livre);
    println!("Ajouté !");
}

fn afficher_livres(bibli: &Vec<Livre>) {
    for livre in bibli {
        println!(
            "{} / {} / {} / {}",
            livre.titre,
            livre.auteur.trim(), 
            livre.annee,
            if livre.disponible { "ok" } else { "non dispo" }
        );
    }
}

fn afficher_dispo(bibli: &Vec<Livre>) {
    for l in bibli.iter() {
        if l.disponible == true {
            println!("{} ({})", l.titre, l.auteur);
        }
    }
}

fn emprunter(bibli: &mut Vec<Livre>) {
    println!("Titre à emprunter ?");
    let mut titre = String::new();
    io::stdin().read_line(&mut titre).unwrap();

    for l in bibli {
        if l.titre == titre.trim() && l.disponible {
            l.disponible = false;
            println!("Emprunté !");
            return;
        }
    }

    println!("Pas trouvé ou indisponible.");
}

fn retourner(bibli: &mut Vec<Livre>) {
    println!("Titre à rendre ?");
    let mut titre = String::new();
    io::stdin().read_line(&mut titre).unwrap();

    let titre = titre.trim();

    for l in bibli {
        if l.titre == titre {
            if l.disponible == false {
                l.disponible = true;
                println!("Retour fait.");
            }
        }
    }


}

fn main() {
    let mut bibli = Vec::new();

    loop {
        println!("");
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Rendre un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles");
        println!("6. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        if choix.trim() == "1" {
            ajouter_livre(&mut bibli);
        } else if choix.trim() == "2" {
            emprunter(&mut bibli);
        } else if choix.trim() == "3" {
            retourner(&mut bibli);
        } else if choix.trim() == "4" {
            afficher_livres(&bibli);
        } else if choix.trim() == "5" {
            afficher_dispo(&bibli);
        } else if choix.trim() == "6" {
            break;
        } else {
            println!("Choix incorrect.");
        }
    }
}
