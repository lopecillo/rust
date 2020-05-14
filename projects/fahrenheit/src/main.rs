fn tofahrenheit(x: f64) -> f64 {
    x*9.0/5.0+32.0
}

fn tocelsius(x: f64) -> f64 {
    (x-32.0)*5.0/9.0
}

fn main() {
    let temperature = 36.5 ;
    println!("Temperature is {}ºC", temperature);

    let fahrenheit = tofahrenheit(temperature);
    println!("Fahrenheit: {}ºF", fahrenheit);

    let celsius = tocelsius(fahrenheit);
    println!("Back to Celsius: {}ºC", celsius);

}
