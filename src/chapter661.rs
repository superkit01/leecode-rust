pub struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = img.len() as i32;
        let m = img[0].len() as i32;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let mut row = Vec::with_capacity(m as usize);
            for _ in 0..m {
                row.push(0);
            }
            result.push(row);
        }

        for i in 0..n {
            for j in 0..m {
                let mut sum: i32 = 0;
                let mut count: i32 = 0;
                for k in i - 1..i + 2 {
                    for l in j - 1..j + 2 {
                        if k >= 0 && k < n && l >= 0 && l < m {
                            count += 1;
                            sum += img[k as usize][l as usize];
                        }
                    }
                }

                result[i as usize][j as usize] = sum / count;
            }
        }

        return result;
    }
}
