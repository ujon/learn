fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{:#?}", element);
    // }
    elements
        .iter()
        .map(|element| format!("{} {}", element, element.len()))
        .for_each(|element| println!("(1) {:#?}", element));
}

fn print_elements_2(elements: &[String]) {
    elements
        .iter()
        .map(|element| format!("{} {}", element, element.len()))
        .for_each(|element| println!("(2) {:#?}", element));
}

fn shorten_string(elements: &mut Vec<String>) {
    elements
        .iter_mut()
        .for_each(|mut element| element.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|element| element.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|element| element.contains(search))
        .map_or(String::from(fallback), |element| element.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("orange"),
        String::from("yellow"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors);

    print_elements_2(&colors[1..3]);

    shorten_string(&mut colors);

    println!("(3) {:#?}", &colors);

    let mut colors_iter = colors.iter();
    println!("(4) {:#?}", colors_iter.next());
    println!("(5) {:#?}", colors_iter.next());
    println!("(6) {:#?}", colors_iter.next());
    println!("(7) {:#?}", colors_iter.next());
    println!("(8) {:#?}", colors_iter.next());
    println!("(9) {:#?}", colors_iter.next());

    let uppercased_colors = to_uppercase(&colors);

    println!("(10) {:#?}", &uppercased_colors);

    let mut dest = vec![];
    move_elements(uppercased_colors, &mut dest);

    println!("(11) {:#?}", &dest);

    let colors = vec![
        String::from("red"),
        String::from("orange"),
        String::from("yellow"),
        String::from("green"),
        String::from("blue"),
    ];

    let find_color = find_color_or(&colors, "ran", "blue");
    println!("(12) {:#?}", &find_color);

    let find_color = find_color_or(&colors, "sdf", "blue");
    println!("(13) {:#?}", &find_color);
}
