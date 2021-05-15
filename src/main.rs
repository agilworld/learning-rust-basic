// declare data structure: tuple struct
// f32 aliases 32bit floating point
struct LatLong(f32, f32);

fn main() {
    // Hello world
    println!("Hello, World!
    I'm a Rustacean!");

    // print sample add one argument
    println!("{0} hours", 24);

    // print some arguments
    println!("{0} from Indonesia, live in {1}, work as {2}", "Dian Afrial", "Batam", "Front-end Engineer");

    // named arguments
    println!("{name} {be} {adverb}", name="Kafin", be="is", adverb="my son");

    // simple data structure
    let _points:LatLong = LatLong(131.021313, -12.12313131);

    // print struct data
    println!("x is {x} and y is {y}", x=_points.0, y=_points.1);

    // Destructure a tuple struct
    let LatLong(lat, long) = _points;

    // print
    println!("latlong contains {:?} and {:?}", lat, long);
}

