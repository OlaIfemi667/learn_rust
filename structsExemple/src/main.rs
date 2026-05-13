#[derive(Debug)] // tres utiles parce qu'il permet d'ajouter des traits aux structs (eg: on peut
                 // utiliser ==  et != plus facilement en nos structs)
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {

    // les fonctions avec &self sont appelés "associated functions" ( des methods )
    fn area(&self) -> u32 // "&self" est un shortcut pour "self: &Self" ce qui fait sens
    {
        self.height * self.width
    }

    fn change_width(&mut self, width: u32){
        self.width = width
    }

    fn can_hold(&self, autre_rectangle: &Rectangle) -> bool { 
        self.width > autre_rectangle.width && self.height > autre_rectangle.height
    }

    // on peut aussi avoir des functions sans &self souvent utiliser comme des constructeurs pour
    // les structs

    fn square(size: u32) -> Self
    {
        Self { width: size, height: size }
    }
}

fn main() {
    let mut my_rectangle = Rectangle {
        width: 30,
        height: 60,
    };

    let second_rectangle = Rectangle {
        width: 20,
        height: 50,
    };

    let third_rectangle = Rectangle {
        width: 40,
        height: 60
    };

    dbg!(&my_rectangle);
    println!("Voici le rectangle {my_rectangle:#?}");
    println!("Rectangle area {}", my_rectangle.area());

    println!("Ici je change le width du rectangle");
    my_rectangle.change_width(50);
    dbg!(&my_rectangle);
    println!("Rectangle area {}", my_rectangle.area());

    println!("\nmy_rectangle peut contenir second_rectangle: {}",
        my_rectangle.can_hold(&second_rectangle));


    println!("\nmy_rectangle peut contenir third_rectangle: {}",
        my_rectangle.can_hold(&third_rectangle));

    let un_carré = Rectangle::square(50);

    println!("\nJ'ai créé un carré avec le constructeur Rectangle::square :{un_carré:#?}");

    println!("Voici son aire {}", un_carré.area());
}


