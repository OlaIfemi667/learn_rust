fn main() {

    //-------------------------Variables and Mutability-------------------------------
    const MA_CONSTANTE: char = 'Z';
    let mut x = 0;

    x += 1;
    println!("La valeur avant le inner scope x = {x}!");
    println!("MA_CONSTANTE = {MA_CONSTANTE}");

    {
        let x = x * 2;
        println!("La valeur de x dans le inner scope x = {x}");
    }

    println!("la valeur de x après le inner scope {x}");

    let to_shadow = "Ola-Iya";
    println!("La variable toShadow avant d'être shadow: {to_shadow}");

    let to_shadow = to_shadow.len();
    println!("La nouvelle variable toShadow : {to_shadow} (len)");
    //-------------------------Data types-------------------------------
    // Scalar type: integers, floating-point numbers, Booleans and characters
    
    //Integers
    let mon_i8: i8 = 127;
    let mon_i16: i16 = 32_767;
    let mon_i32: i32 = 2147483647;
    let mon_i64: i64 = 9223372036854775807;
    let mon_isize: isize = 9223372036854775807;

    println!("\nmon_i8 = {mon_i8}");
    println!("mon_i16 = {mon_i16}");
    println!("mon_i32 = {mon_i32}");
    println!("mon_i64 = {mon_i64}");
    println!("mon_isize = {mon_isize}");

    let mon_i8: i8 = mon_i8.wrapping_add(1);
    let mon_i16: i16 = mon_i16.wrapping_add(1);
    let mon_i32: i32 = mon_i32.wrapping_add(1);
    let mon_i64: i64 = mon_i64.wrapping_add(1);
    let mon_isize: isize = mon_isize.wrapping_add(1);
    
    println!("\n J'ai choisi les plus grand nombre de chaque type et je leur ai ajouté  1 en utilisant wrapping_add donc normalement il devrait etre égal au minimum de leur type.\n");

    println!("mon_i8 = {mon_i8}");
    println!("mon_i16 = {mon_i16}");
    println!("mon_i32 = {mon_i32}");
    println!("mon_i64 = {mon_i64}");
    println!("mon_isize = {mon_isize}");


    let mon_i8_binaire: i8 = 0b0000_1111;
    let mon_bool: bool = false;

    println!("\nmon_i8_binaire {mon_i8_binaire}");
    println!("mon_bool = {mon_bool}");

    let mon_f64: f64 = 2.0;
    let mon_f32: f32 = 5.6;
    println!("\nmon_f64 = {mon_f64}");
    println!("mon_f32 = {mon_f32}");

    let mon_char: char = 'c';
    let mon_char_emoticone = '😻';

    
    println!("\nmon_char = {mon_char}");
    println!("mon_char_emoticone = {mon_char_emoticone}");

    let mon_tuple: (i8, char) = (22, 'J');
    let (x, y) = mon_tuple;
    println!("\nmon_tuple = ({x},{y})");


    let mon_array: [char; 7] = ['O','l','a','-','I','y','a'];
    let element = mon_array[3];
    println!("mon_array[3] = {element}");

    ma_fonction("BlackNigga", 125);


}

fn ma_fonction(user: &str, age: i8)
{
    println!("------------------------");
    println!("\nLa fonction de Ola-Iya est actuellement utiliseé par -> {user} agé de {age}");
    let five = cinq();
    println!("La fonction cinq: {five}");
    let plus = plus_one(five);
    println!("cinq + 1: {plus}");
}


fn cinq() -> i8
{
    5
}

fn plus_one(x: i8) -> i8
{
    x + 1
}
