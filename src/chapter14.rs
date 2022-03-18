pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut max = strs[0].clone();


        for (index,elem) in strs.iter().enumerate() {
            if index==0{
                continue;
            }

            let mut i=0;
            while i< max.len() && i< elem.len(){
              if  max.as_bytes()[i]!=elem.as_bytes()[i]{
                  break;
              }
                i+=1;
            }
            max=max[0..i].to_string();
        }

        return max;
    }
}
