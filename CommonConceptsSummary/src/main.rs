fn main() {
    let temp_farenheit: f32 = 50.0;
    let temp_celcius = fahrenheit_to_celcius(temp_farenheit);

    println!("{temp_farenheit} °F -> {temp_celcius} °C");
    let n: i16 = 15;
    let nuombre = fibonacci(n);
    println!("Voici le {n}ieme nombre de la séquence de fibonacci est {nuombre}");

    chrismas_song();
}


fn fahrenheit_to_celcius(x: f32) -> f32
{
    (x - 32.0) * (5.0 / 9.0)
}

fn fibonacci(n : i16) -> i16
{
    if n == 0
    {
        return 0;
    }
    if n == 1
    {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn chrismas_song()
{
    const PRESENT: [&str; 12] = 
        ["A partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming" ];
    const DAYS: [&str; 12] = 
        ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    println!("This is a chrismas song");

    for i in 0..12
    {
        let day = DAYS[i];
        println!("On the {day} day of Christmas my true love sent to me");

        for j in (0..i+1).rev()
        {
            let present = PRESENT[j];
            println!("{present}");
        }
        println!("\n\n");
    }


}
