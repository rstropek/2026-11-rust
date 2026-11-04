mod tower;

fn main() {
    let result_array = tower::calculate_tower(tower::BASE);
    for (i, op, operand, result) in result_array {
        println!("{i} {op} {operand} = {result}");
    }
}
