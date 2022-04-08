pub struct Solution {}
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dir: Vec<&str> = path.split("/").collect();
        
        dir=dir.iter().map(|i| i.to_owned()).filter(|x| x.to_owned() != "").collect::<Vec<&str>>();

        let mut result:Vec<&str>=vec![];

        for e in dir.iter(){

            if e.to_owned() == "."{
                continue;
            }

            if e.to_owned() ==".."{
                if result.len()!=0 {
                    result.pop();
                }
                continue;
            }
            
            result.push(e);
        }

        return String::from("/") + result.join("/").as_str();
    }
}
