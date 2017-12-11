

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>; // 0.0 is f32, 0 is i32

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut result : Matrix = Matrix::new();

    for i in 0..mat1.len() {
        result.push(Vec::new());
        for j in 0..mat2[0].len() {
            let mut sum : f32 = 0.0;
            for k in 0..mat2.len() {
                sum += mat1[i][k] * mat2[k][j];
            }
            result[i].push(sum);
        }
    }

    result
}

