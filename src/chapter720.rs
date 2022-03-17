pub struct Solution{}

impl Solution {
    pub fn longest_word(words:  Vec<String>) -> String {

        if words.len()==1 {
            if words[0].len()!=1 {
                return String::from("")
            }else{
             return words[0].clone();
            }
        }

        let mut mut_words=words;
        mut_words.sort();
        println!("{:?}",mut_words);


        let mut result:Vec<String>=Vec::new();
        Self::digui(0, mut_words.len(), &mut mut_words, &mut result);

        let mut max=String::from("");
        for elem in result.iter() {
            if elem.len()>max.len(){
                max=elem.clone();
            }
        }


        return max;
    }


    pub fn digui( mut start:  usize,len:usize, words: &mut Vec<String>,result:&mut  Vec<String> ){
        if start>=len {
            return;
        }
        if words[start as usize].len()!=1 {
            Self::digui(start+1, len, words,result);
            return;
        }
        
        result.push(words[start].clone());
        println!("{:?}",result);

        while start <len-1 {
            if words[start+1].len() -words[start].len()!=1 || words[start+1].starts_with(words[start].as_str()) {
               Self:: digui(start+1, len, words,result);
               return;
            }
            result.push(words[start+1].clone());
            start+=1;
        }


    }
}