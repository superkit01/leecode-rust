pub struct Solution{}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut add=1;
        let mut i =digits.len() as i32 -1 ;
        let mut result:Vec<i32> =Vec::new();
        while  i>=0 {
           let sum= digits[i as usize]+add;
            result.push(sum%10);
            add=sum/10;
            i-=1;
        }
        if add!=0{
            result.push(add)
        }
        result.reverse();
        return result;

    }
}