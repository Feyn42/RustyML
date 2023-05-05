
// Dot Product
pub fn dot(vec1: &[f64], vec2: &[f64]) -> f64 {

    assert!(vec1.len() == vec2.len());

    let mut result = 0.0;

    for i in 0..vec1.len() {
        result += vec1[i] * vec2[i]
    }

    return result;
    
}