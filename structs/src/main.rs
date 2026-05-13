struct Human
{
    name: String,
    email: String,
    height: i16,
    weight: f32,
    is_a_man: bool,
}

struct Belt(i32, i32, i32);


fn main() {
    let ola_iya = Human
    {
        name: String::from("Ola-Iya BANCOLE"),
        email: String::from("ola@mail.bj"),
        height: 186,
        weight: 75.6,
        is_a_man: true,
    };

    let jean_de_dieu = Human
    {
        email: String::from("jean@mail.bj"),
        ..ola_iya
    };

    let name = jean_de_dieu.name;
    let email = jean_de_dieu.email;
    println!("Voici le \"Human\" {name} et son mail {email}");

    let default_belt = Belt(255, 255, 255);

    let Belt(x, y, z) = default_belt;

    println!("Default belt color code: {x} {y} {z}");


}
