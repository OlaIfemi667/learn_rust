fn main() {
    {
        let s = "Ola";
        let x = String::from("-Iya");
        println!("Le variable s = {s} n'est valide que dans ce scope");
        println!("La variable x = {x} n'est valide que dans ce scope");
    } // s et x ne sont plus valide a partir d'ici

    // println!("{x} {s}");  <- crée une erreur de compilatio

    let s1 = String::from("Hello");

    let mut s2 = s1.clone();

    println!("{s1}");
    s2.push_str(", Ola-Iya");

    println!("{s2}");

    prendre_possession(s2);

    let y: i8 = 100;

    faire_copie(y);

    println!("{y}");
    // println!("{s2}"); <- crée une erreur de compilation
    //
    let le_string = String::from("Mon pain");

    let un_autre_string = prendre_et_rendre(le_string);

    println!("\"{un_autre_string}\" est retourné par la fonction prendre_et_rendre(le_string)");

    let un_tuple: (String, usize) = rendre_tuple(un_autre_string);

    let mut a = un_tuple.0;
    let b = un_tuple.1;
    println!("Nous pouvons rendre un tuple ({a}, {b})");


    println!("\nVoici le contenu de un_autre_string: {a}");

    let autre_len = borrow_it(&a);
    println!("Voici sa taille a travers une fonction {autre_len}");

    change_it(&mut a);

    println!("Voici la nouvelle version du string a {a}");

    let nom = String::from("Ola-Iya");
    let name_slice = &nom[2..];
    println!("{name_slice}");

    assert_eq!(name_slice, "a-Iya");
    
}

fn prendre_possession(un_string: String)
{
    println!("la variable passée :\"{un_string}\" ne pourra plus etre accessible après cette fonction");

}


fn faire_copie(un_entier: i8)
{
    println!("Ce entier est copier dans cette fonction :{un_entier}, il serait disponible apres exécution de la function");
}


fn prendre_et_rendre(un_string: String) -> String
{
    un_string
}

fn rendre_tuple(un_string: String) -> (String, usize)
{
    let length = un_string.len();
    (un_string, length)
}

fn borrow_it(mon_string: &String) -> usize
{
    mon_string.len()
}

 fn change_it(mon_string: &mut String)
 {
     mon_string.push_str("::a été changé::");
 }
