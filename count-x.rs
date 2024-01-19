/* CODEWARS | Count by X  */
fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut i = 1; 
    let mut a:Vec<u32> = vec![];
    while i <= n {
        a.push(i*x);
        i += 1;
    }
    
    return a;
}