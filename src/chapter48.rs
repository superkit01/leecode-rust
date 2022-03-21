pub struct Solution{}




impl Solution{
    pub fn rotate( matrix: &mut Vec<Vec<i32>>){
        //上下翻转
        let mut i=0;
        let lenth= matrix.len();
        if lenth==1 {
            return;
        }


        while i< lenth/2{
            matrix.swap(i,lenth-1-i);
            i+=1;
        }

        //主对角线翻转
        i=0;
        while i< lenth{
          let mut j=i+1;
            while j<lenth {
                let tmp=matrix[i][j];
                matrix[i][j]=matrix[j][i];
                matrix[j][i]=tmp;
                j+=1;
            }
            i+=1;
        }
    }




}