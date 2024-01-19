/* CODEWARS | Sum of Cubes */
fn sum_cubes(n: u32) -> u32 {
  let mut s:u32 = 0; 
  let mut i:u32 = 1; 
    
  while(i<=n){
      s += i.pow(3); 
      i+=1;
  }
  
  return s;
}