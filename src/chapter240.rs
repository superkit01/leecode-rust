pub struct Solution {}

//醍醐灌顶： 矩阵右上角来看就是一个binary search tree 

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut i = 0 ;
        let mut j = m as i32 - 1;
        while j >= 0 && i < n {
            if matrix[i][j as usize] > target {
                j-=1;
            }else if  matrix[i][j as usize] < target{
                i+=1;
            }else {
                return true;
            }
        }

        return false;
    }
}
