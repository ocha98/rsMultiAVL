use crate::avl::MultiAVL;
use rand::{ SeedableRng, seq::SliceRandom, rngs::StdRng };

fn setup_tree(values: &Vec<i32>) -> MultiAVL<i32> {
    let mut tree = MultiAVL::new();
    for i in values {
        tree.insert(*i);
    }
    tree
}

#[test]
fn test_contains() {
    let mut tree = MultiAVL::new();
    tree.insert(1);
    assert!(tree.contains(1));
    assert!(!tree.contains(0));
}

#[test]
fn test_size() {
    let mut tree = MultiAVL::new();
    assert_eq!(tree.size(), 0);
    tree.insert(1);
    assert_eq!(tree.size(), 1);
    tree.erase(1);
    assert_eq!(tree.size(), 0);
}

// == インサートテスト ==
#[test]
fn test_insert_ascending_order() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    for i in 0..n {
        assert_eq!(tree.contains(i), false);
        tree.insert(i);
    }
    for i in 0..n {
        assert_eq!(tree.contains(i), true);
    }

    assert!(tree.check_consistent().is_ok());
    assert_eq!(tree.size(), n as usize);
}

#[test]
fn test_insert_descending_order() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    for i in (0..n).rev() {
        assert_eq!(tree.contains(i), false);
        tree.insert(i);
    }
    for i in 0..n {
        assert_eq!(tree.contains(i), true);
    }

    assert!(tree.check_consistent().is_ok());
    assert_eq!(tree.size(), n as usize);
} 

#[test]
fn test_insert_shuffled() {
    let n = 1_000;
    let mut nums: Vec<i32> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);

    let mut tree = MultiAVL::new();
    for i in &nums {
        tree.insert(*i);
        assert!(tree.check_consistent().is_ok());
    }

    assert_eq!(tree.size(), n as usize);

    for i in &nums {
        assert!(tree.contains(*i));
    }
}

// == 削除テスト ==
fn do_erase_test(test_cases: &Vec<(Vec<i32>, i32)>) {
    for (case_num, (values, target)) in test_cases.iter().enumerate() {
        let mut tree = setup_tree(values);
        for i in values {
            tree.contains(*i);
        }

        tree.erase(*target);
        assert!(!tree.contains(*target), "case {} failed", case_num);
        for i in values {
            if *i == *target { continue; }
            assert!(tree.contains(*i), "case {} failed", case_num);
        }

        assert_eq!(tree.size(), values.len() - 1, "case {} failed", case_num);
        assert!(tree.check_consistent().is_ok(), "case {} failed", case_num);
    }
}

#[test]
fn test_erase_no_child() {
    /*
    [case 0] del: 1
        削除前の木       =>     削除後の木
        
            2                     2
           /
          1  
    
    [case 1] del: 2
        削除前の木       =>     削除後の木
        
            1                     1
             \
              2  

    [case 2] del: 1
        削除前の木       =>     削除後の木
        
            1   
    */

    let test_cases = vec![
        (vec![2, 1], 1),
        (vec![1, 2], 2),
        (vec![1], 1),
    ];
    do_erase_test(&test_cases);
}

#[test]
fn test_erase_one_child() {
    /*
    [case 1] del: 4
        削除前の木       =>     削除後の木
        
             2                     2
            / \                   / \
           1   4                 1   3
              /
             3

    [case 2] del: 3
        削除前の木       =>     削除後の木
        
             2                     2
            / \                   / \
           1   3                 1   4
                \
                 4

    [case 3] del: 2
        削除前の木       =>     削除後の木
        
             3                          3
            / \                        / \
           2   4                      1   4
          /
         1
    
    [case 4] del: 1
        削除前の木       =>     削除後の木
        
             3                          3
            / \                        / \
           1   4                      2   4
            \
             2
    [case 5] del: 1
        削除前の木       =>     削除後の木
        
             1                          2
              \                          
               2    
 
    [case 6] del 2
        削除前の木       =>     削除後の木
        
            2                     1
           /                                        
          1           
    */

    let test_cases = vec![
        (vec![2, 1, 4, 3], 4),
        (vec![2, 1, 3, 4], 3),
        (vec![3, 4, 2, 1], 2),
        (vec![3, 4, 1, 2], 1),
        (vec![1, 2], 1),
        (vec![2, 1], 2),
    ];
    do_erase_test(&test_cases);
}

#[test]
fn test_erase_two_children() {
    /*
    [case 0] del: 2
        削除前の木         =>     削除後の木
    
            2                        1
           / \                          \
          1   3                          3
    
    [case 1] del: 3
        削除前の木         =>     削除後の木
    
            3                            2
           / \                          / \
          1   4                        1   4
           \                                \
            2                                2
    
    [case 2] del: 2
        削除前の木         =>     削除後の木
    
            4                            4
           / \                          / \
          2   5                        1   5
         / \                            \
        1   3                            3
    
    [case 3] del: 4
        削除前の木         =>     削除後の木
    
            6                            6
           / \                          / \
          4   7                        3   7
         / \   \                      / \   \
        2   5   8                    2   5   8
       / \       =>                  /  
      1   3                        1  
    
    */

    let test_cases = vec![
        (vec![2, 1,3], 2),
        (vec![3, 1, 4, 2], 3),
        (vec![4, 2, 5, 1, 3], 2),
        (vec![4, 2, 5, 1, 3], 4),
    ];
    do_erase_test(&test_cases);
}

#[test]
fn test_erase_not_exist() {
    let mut tree = MultiAVL::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);

    assert_eq!(tree.size(), 3);
    tree.erase(10);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_erase_shuffled() {
    let n = 1_000;
    let mut nums: Vec<i32> = (0..n).collect();
    let mut tree = MultiAVL::new();
    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);

    for i in &nums {
        tree.insert(*i);
    }

    let mut expected_size = nums.len();
    assert_eq!(tree.size(), expected_size);
    nums.shuffle(&mut rng);
    for i in &nums {
        assert_eq!(tree.contains(*i), true);
        tree.erase(*i);
        expected_size -= 1;
        assert_eq!(tree.contains(*i), false);
        assert_eq!(tree.size(), expected_size);

        assert!(tree.check_consistent().is_ok());
    }
    assert_eq!(tree.size(), 0);
}

// == 回転処理テスト == 
fn do_rotate_test(test_cases: &Vec<Vec<i32>>) {
    for (case_num, values) in test_cases.iter().enumerate() {
        let tree = setup_tree(values);
        for i in values {
            tree.contains(*i);
        }
        assert!(tree.check_consistent().is_ok(), "case {} failed", case_num);
    }
}

#[test]
fn test_rotate_left() {
    /*
    [case 0]
        回転前の木         =>     回転後の木
            1                            2
             \                          / \
              2                        1   3
               \
                3
    
    [case 1]
        回転前の木         =>     回転後の木
            2                            2
           / \                          / \
          1   3                        1   4
               \                          / \
                4                        3   5
                 \                      
                  5                     
    [case 2]
        回転前の木         =>     回転後の木
            4                        4
           / \                      / \
          1   5                    2   5
           \                      / \
            2                    1    3
             \                      
              3                 
    */

    let test_cases = vec![
        vec![1, 2, 3],
        vec![2, 1, 3, 4, 5],
        vec![4, 5, 1, 2, 3],
    ];
    do_rotate_test(&test_cases);
}

#[test]
fn test_rotate_right() {
    /*
    [case 0]
        回転前の木         =>        回転後の木
            3                           2
           /                           / \
          2                           1   3
         /  
        1     

    [case 1]
        回転前の木         =>        回転後の木
              4                         4
             / \                       / \
            3   5                     2   5
           /                         / \
          2                         1   3
         /
        1

    [case 2]
        回転前の木         =>        回転後の木
            2                           2
           / \                         / \
          1   5                       1   4
             /                           / \
            4                           3   5
           /
          3
        
    */

    let test_cases = vec![
        vec![3, 2, 1],
        vec![4, 5, 3, 2, 1],
        vec![2, 1, 5, 4, 3],
    ];
    do_rotate_test(&test_cases);

}

#[test]
fn test_double_rotation_left_right() {
    /*
    [case 1]
        回転前の木         =>        回転後の木
            3                           2
           /                           / \
          1                           1   3
           \
            2

    [case 2]
         回転前の木         =>        回転後の木
            5                           3
           / \                         / \
          2   6                       2   5
         / \                         /   / \
        1   3                       1   4   6
             \
              4   

    [case 3]
        回転前の木         =>        回転後の木
            5                           4
           / \                         / \
          2   6                       2   5
         / \                         / \   \
        1   4                       1   3   6
           /
          3
    */

    let test_cases = vec![
        vec![3, 1, 2],
        vec![5, 6, 2, 1, 3, 4],
        vec![5, 6, 2, 1, 4, 3],
    ];
    do_rotate_test(&test_cases);
}

#[test]
fn test_double_rotation_right_left() {
    /*
    [case 0]
        回転前の木         =>        回転後の木
        
            1                           2
             \                         / \
              3                       1   3
             /     
            2        
    
    [case 1]
        回転前の木         =>        回転後の木
            2                           4
           / \                         / \
          1   5                       2   5
             / \                     / \   \
            4   6                   1   3   6
           /
          3 

    [case 2]
        回転前の木         =>        回転後の木
            2                           3
           / \                         / \
          1   5                       2   5
             / \                     /   / \
            3   6                   1   4   6
             \  
              4     
    */

    let test_cases = vec![
        vec![1, 3, 2],
        vec![2, 1, 5, 4, 6, 3],
        vec![2, 1, 5, 3, 6, 4],
    ];
    do_rotate_test(&test_cases);
}

// == イテレータテスト == 
#[test]
fn test_iter_order() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    let mut nums:Vec<i32> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);
    for i in &nums {
        tree.insert(*i);
        tree.insert(*i);
        tree.insert(*i);
    }

    let mut iter = tree.iter();
    for i in 0..n {
        assert_eq!(iter.next().unwrap(), i);
        assert_eq!(iter.next().unwrap(), i);
        assert_eq!(iter.next().unwrap(), i);
    }

    assert_eq!(iter.next(), None);
}

// 最大最小テスト
#[test]
fn test_max_value() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    let mut nums:Vec<i32> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);
    let mut mx = -1;
    for i in &nums {
        mx = mx.max(*i);
        tree.insert(*i);
        assert_eq!(tree.max_value().unwrap(), mx);
    }

    assert_eq!(tree.max_value().unwrap(), n-1);
    nums.shuffle(&mut rng);
    loop {
        let v = nums.pop().unwrap();
        tree.erase(v);
        if nums.is_empty() { break; }
        let mx = nums.iter().max().unwrap();
        assert_eq!(tree.max_value().unwrap(), *mx);
    }

    assert_eq!(tree.max_value(), None);
}

#[test]
fn test_min_value() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    let mut nums:Vec<i32> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);
    let mut mi = std::i32::MAX;
    for i in &nums {
        mi = mi.min(*i);
        tree.insert(*i);
        assert_eq!(tree.min_value().unwrap(), mi);
    }

    assert_eq!(tree.min_value().unwrap(), 0);
    nums.shuffle(&mut rng);
    loop {
        let v = nums.pop().unwrap();
        tree.erase(v);
        if nums.is_empty() { break; }
        let mx = nums.iter().min().unwrap();
        assert_eq!(tree.min_value().unwrap(), *mx);
    }

    assert_eq!(tree.min_value(), None);
}

#[test]
fn test_multi_insert() {
    let n = 1_000;
    let mut tree = MultiAVL::new();
    let mut nums = vec![];
    for i in 0..n {
        for _ in 0..3 {
            nums.push(i);
        }
    }

    let mut rng = StdRng::seed_from_u64(0);
    nums.shuffle(&mut rng);
    for i in &nums {
        tree.insert(*i as i32)
    }

    assert_eq!(tree.size(), 3*n);
    assert!(tree.check_consistent().is_ok());

    let mut iter = tree.iter();
    nums.sort();
    for i in &nums {
        assert_eq!(iter.next().unwrap(), *i as i32);
    }

    assert_eq!(iter.next(), None)
}

#[test]
fn test_allow_multi() {
    let mut tree = MultiAVL::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(1);
    tree.insert(2);
    
    assert_eq!(tree.size(), 4);
    assert_eq!(tree.contains(1), true);
    assert_eq!(tree.contains(2), true);
    assert!(tree.check_consistent().is_ok());
}
