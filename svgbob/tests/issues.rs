extern crate svgbob;

//#[test]
fn issue15(){
    use svgbob::{Grid,Settings};
    let input = "+--+\n| |\n+--+";
    let g = Grid::from_str(input, &Settings::separate_lines());
    println!("grid: {:?}", g);
    let result = g.get_svg_nodes_only();
    println!("grid: {:?}", g.to_string());
    let expected = "<line x1=\"0\" x2=\"8\" y1=\"8\" y2=\"8\"/><line x1=\"0\" x2=\"8\" y1=\"8\" y2=\"8\"/>";
    println!("expected: {}", expected);
    println!("result: {}", result);
    assert_eq!(result, expected);
}
