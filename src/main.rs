// mod chapter38;

// mod chapter21;

// mod chapter39;

// mod chapter40;

// mod chapter26;
// use chapter26::*;

// mod chapter2190;

// mod chapter46;

// mod chapter14;

// mod chapter720;

// mod chapter13;

// mod chapter2043;
// use chapter2043::Bank;

// mod chapter720;

// mod chapter606;
// use chapter606::*;
// use std::cell::RefCell;
// use std::rc::Rc;

// mod chapter27;

// mod chapter48;

// mod chapter35;
// mod chapter49;

// mod chapter67;

// mod chapter240;

// mod chapter661;

// mod chapter80;

// mod chapter94;
// use chapter94::*;
// use std::cell::RefCell;
// use std::rc::Rc;

// mod chapter172;

// mod chapter36;
// mod chapter682;
// mod chapter693;

// mod chapter66;

// mod chapter50;

// mod chapter56;

// mod chapter71;

// mod chapter804;

// mod chapter97;

mod chapter908;

fn main() {
    println!("hello world");

    // ## chapter 38
    // let result = chapter38::Solution::count_and_say(4);
    // println!("{}", result);

    //## chapter 21
    // let list1: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
    //     1,
    //     Some(Box::new(ListNode::new(
    //         2,
    //         Some(Box::new(ListNode::new(4, None))),
    //     ))),
    // )));
    // println!("{:?}", list1);

    // let list2: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
    //     1,
    //     Some(Box::new(ListNode::new(
    //         3,
    //         Some(Box::new(ListNode::new(4, None))),
    //     ))),
    // )));
    // println!("{:?}", list2);
    // let result21 = Chapter21::Solution::merge_two_lists(list1, list2);
    // println!("{:?}", result21)

    //# chapter40
    // let candidates = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    // let target = 30;
    // let result: Vec<Vec<i32>> = Solution::combination_sum(candidates, target);
    // println!("{:?}", result)

    //# chapter26
    //  let mut nums = vec![1,1,2];
    //  let count=Chapter26::Solution::remove_duplicates(&mut nums);
    //  println!("{:?}",nums);
    //  println!("{:?}",count);

    //# chapter2190
    // let nums = vec![1,999,3,999,5,999,7,999,9,999,11,999,13,999,15,999,17,999,19,999,21,999,23,999,25,999,27,999,29,999,31,999,33,999,35,999,37,999,39,999,41,999,43,999,45,999,47,999,49,999,51,999,53,999,55,999,57,999,59,999,61,999,63,999,65,999,67,999,69,999,71,999,73,999,75,999,77,999,79,999,81,999,83,999,85,999,87,999,89,999,91,999,93,999,95,999,97,999,99,999,101,999,103,999,105,999,107,999,109,999,111,999,113,999,115,999,117,999,119,999,121,999,123,999,125,999,127,999,129,999,131,999,133,999,135,999,137,999,139,999,141,999,143,999,145,999,147,999,149,999,151,999,153,999,155,999,157,999,159,999,161,999,163,999,165,999,167,999,169,999,171,999,173,999,175,999,177,999,179,999,181,999,183,999,185,999,187,999,189,999,191,999,193,999,195,999,197,999,199,999,201,999,203,999,205,999,207,999,209,999,211,999,213,999,215,999,217,999,219,999,221,999,223,999,225,999,227,999,229,999,231,999,233,999,235,999,237,999,239,999,241,999,243,999,245,999,247,999,249,999,251,999,253,999,255,999,257,999,259,999,261,999,263,999,265,999,267,999,269,999,271,999,273,999,275,999,277,999,279,999,281,999,283,999,285,999,287,999,289,999,291,999,293,999,295,999,297,999,299,999,301,999,303,999,305,999,307,999,309,999,311,999,313,999,315,999,317,999,319,999,321,999,323,999,325,999,327,999,329,999,331,999,333,999,335,999,337,999,339,999,341,999,343,999,345,999,347,999,349,999,351,999,353,999,355,999,357,999,359,999,361,999,363,999,365,999,367,999,369,999,371,999,373,999,375,999,377,999,379,999,381,999,383,999,385,999,387,999,389,999,391,999,393,999,395,999,397,999,399,999,401,999,403,999,405,999,407,999,409,999,411,999,413,999,415,999,417,999,419,999,421,999,423,999,425,999,427,999,429,999,431,999,433,999,435,999,437,999,439,999,441,999,443,999,445,999,447,999,449,999,451,999,453,999,455,999,457,999,459,999,461,999,463,999,465,999,467,999,469,999,471,999,473,999,475,999,477,999,479,999,481,999,483,999,485,999,487,999,489,999,491,999,493,999,495,999,497,999,499,999,501,999,503,999,505,999,507,999,509,999,511,999,513,999,515,999,517,999,519,999,521,999,523,999,525,999,527,999,529,999,531,999,533,999,535,999,537,999,539,999,541,999,543,999,545,999,547,999,549,999,551,999,553,999,555,999,557,999,559,999,561,999,563,999,565,999,567,999,569,999,571,999,573,999,575,999,577,999,579,999,581,999,583,999,585,999,587,999,589,999,591,999,593,999,595,999,597,999,599,999,601,999,603,999,605,999,607,999,609,999,611,999,613,999,615,999,617,999,619,999,621,999,623,999,625,999,627,999,629,999,631,999,633,999,635,999,637,999,639,999,641,999,643,999,645,999,647,999,649,999,651,999,653,999,655,999,657,999,659,999,661,999,663,999,665,999,667,999,669,999,671,999,673,999,675,999,677,999,679,999,681,999,683,999,685,999,687,999,689,999,691,999,693,999,695,999,697,999,699,999,701,999,703,999,705,999,707,999,709,999,711,999,713,999,715,999,717,999,719,999,721,999,723,999,725,999,727,999,729,999,731,999,733,999,735,999,737,999,739,999,741,999,743,999,745,999,747,999,749,999,751,999,753,999,755,999,757,999,759,999,761,999,763,999,765,999,767,999,769,999,771,999,773,999,775,999,777,999,779,999,781,999,783,999,785,999,787,999,789,999,791,999,793,999,795,999,797,999,799,999,801,999,803,999,805,999,807,999,809,999,811,999,813,999,815,999,817,999,819,999,821,999,823,999,825,999,827,999,829,999,831,999,833,999,835,999,837,999,839,999,841,999,843,999,845,999,847,999,849,999,851,999,853,999,855,999,857,999,859,999,861,999,863,999,865,999,867,999,869,999,871,999,873,999,875,999,877,999,879,999,881,999,883,999,885,999,887,999,889,999,891,999,893,999,895,999,897,999,899,999,901,999,903,999,905,999,907,999,909,999,911,999,913,999,915,999,917,999,919,999,921,999,923,999,925,999,927,999,929,999,931,999,933,999,935,999,937,999,939,999,941,999,943,999,945,999,947,999,949,999,951,999,953,999,955,999,957,999,959,999,961,999,963,999,965,999,967,999,969,999,971,999,973,999,975,999,977,999,979,999,981,999,983,999,985,999,987,999,989,999,991,999,993,999,995,999,997,999,999,999];
    // let target = 999;
    // let result =Chapter2190::Solution::most_frequent(nums, target);
    // println!("{}", result)

    //chapter720
    // let words: Vec<String> = vec![
    //     String::from("a"),
    //     String::from("banana"),
    //     String::from("aa"),
    //     String::from("aaa"),
    //     String::from("aae"),
    // ];
    // let result= chapter720::Solution::longest_word(words);
    // println!("{}",result);

    //chapter2043
    // let bank: Bank = Bank::new(vec![10, 100, 20, 50, 30]);
    // bank.withdraw(3, 10);
    // bank.transfer(5, 1, 20);
    // bank.deposit(5, 20);
    // bank.transfer(3, 4, 15);
    // bank.withdraw(10, 50);

    //chapter606
    //      1
    //     /   \
    //    2     3
    //   /
    //  4
    //
    // 输出: "1(2(4))(3)"
    // let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 1,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    //         right: None,
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode ::new(3)))),
    // })));
    // let result= chapter606::Solution::tree2str(root);
    // println!("{}",result)

    //chapter46
    // let nums:Vec<i32> = vec![1,2,3];
    // let result=chapter46::Solution::permute(nums);
    // print!("{:?}",result)

    //chapter14
    // let strs: Vec<String> = vec![
    //     String::from("flower"),
    //     String::from("flow"),
    //     String::from("floor"),
    //     String::from("flight"),
    // ];
    // let result = chapter14::Solution::longest_common_prefix(strs);
    // println!("{}", result)

    //chapter13
    // let s= String::from("MCMXCIV");
    // let result=chapter13::Solution::roman_to_int(s);
    // println!("{}",result);

    //chapter27
    // let mut  nums=vec![3,2,2,3];
    // let target =3;
    // let result= chapter27::Solution::remove_element(&mut nums, target);
    // println!("{}",result)

    //chapter48
    // let mut matrix=vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    // chapter48::Solution::rotate(&mut matrix);
    // println!("{:?}",matrix);

    //chapter35
    // let nums = vec![1, 3];
    // let target = 0;
    // let result = chapter35::Solution::search_insert(nums, target);
    // println!("{}", result)

    //chapter49
    // let strs=vec![String::from("eat"),String::from("tea"),String::from("tan"),String::from("ate"),String::from("nat"),String::from("bat")];
    // let result = chapter49::Solution::group_anagrams(strs);
    // println!("{:?}",result);

    //chapter67
    // let result=chapter67::Solution::add_binary(String::from("1111"),  String::from("11"));
    // println!("{}",result);

    //chapter240
    // let matrix=vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]];
    // let result=chapter240::Solution::find_number_in2_d_array(matrix, 5);
    // println!("{}",result)

    //chapter661
    // let matrix = vec![
    //     vec![1, 4, 7, 11, 15],
    //     vec![2, 5, 8, 12, 19],
    //     vec![3, 6, 9, 16, 22],
    //     vec![10, 13, 14, 17, 24],
    //     vec![18, 21, 23, 26, 30],
    // ];
    // let result = chapter661::Solution::image_smoother(matrix);
    // println!("{:?}",result);

    //chapter80
    // let mut nums = vec![1, 2, 2];
    // let result = chapter80::Solution::remove_duplicates(&mut nums);
    // println!("{:?}", result)

    //chapter94
    // let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 1,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    //         right: None,
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode ::new(3)))),
    // })));
    // let result= chapter94::Solution::inorder_traversal(root);
    // println!("{:?}",result)

    //chapter172
    // let result=chapter172::Solution::trailing_zeroes(30);
    // println!("{:?}",result);

    //chapter36
    // let matrix=vec![vec!['5','3','.','.','7','.','.','.','.'],
    //                 vec!['6','.','.','1','9','5','.','.','.'],
    //                 vec!['.','9','8','.','.','.','.','6','.'],
    //                 vec!['8','.','.','.','6','.','.','.','3'],
    //                 vec!['4','.','.','8','.','3','.','.','1'],
    //                 vec!['7','.','.','.','2','.','.','.','6'],
    //                 vec!['.','6','.','.','.','.','2','8','.'],
    //                 vec!['.','.','.','4','1','9','.','.','5'],
    //                 vec!['.','.','.','.','8','.','.','7','9']];
    // let result=chapter36::Solution::is_valid_sudoku(matrix);
    // println!("{}",result)

    //chapter682
    // let ops: Vec<String> = vec![
    //     String::from("5"),
    //     String::from("-2"),
    //     String::from("4"),
    //     String::from("C"),
    //     String::from("D"),
    //     String::from("9"),
    //     String::from("+"),
    //     String::from("+"),
    // ];
    // let result = chapter682::Solution::cal_points(ops);
    // println!("{}", result);

    //chapter693
    // let result = chapter693::Solution::has_alternating_bits(5);
    // println!("{}", result);

    // chapter66
    // let result=chapter66::Solution::plus_one(vec![1,2,3]);
    // println!("{:?}",result);

    //chapter50
    // let result=chapter50::Solution::my_pow(3.0,3);
    // println!("{}",result)

    //chapter56
    // let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    // let result = chapter56::Solution::merge(intervals);
    // println!("{:?}", result)

    //chapter71
    // let result=chapter71::Solution::simplify_path(String::from("/root/home/data/../data/home"));
    // println!("{}",result)
    
    //chapter804
    // let words=vec![String::from("gin"), String::from("zen"), String::from("gig"), String::from("msg")];
    // let result = chapter804::Solution::unique_morse_representations(words);
    // println!("{}",result)

    //chapter97
    // let result=chapter97::Solution::is_interleave(String::from("aabcc"), String::from("dbbca"),  String::from("aadbbcbcac"));
    // println!("{}",result)

    //chapter908
    let result=chapter908::Solution::smallest_range_i(vec![1,3,6],3);
    println!("{}",result)

}
