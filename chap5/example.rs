fn helper(
    heights: &Vec<Vec<i32>>,
    visited_ocean: &mut Vec<Vec<bool>>,
    to_ocean: &mut Vec<Vec<bool>>,
    index_a: usize,
    index_b: usize,
) {
    if index_a >= heights.len() 
        || index_b >= heights[0].len() 
        || visited_ocean[index_a][index_b] {
        return;
    }
    visited_ocean[index_a][index_b] = true;
    let directions = [(1, 0), (0, 1), (usize::MAX, 0), (0, usize::MAX)]; 
    // 使用 usize 溢出表示 -1
    for &(da, db) in &directions {
        let new_a = index_a.wrapping_add(da); // 使用 wrapping_add 处理溢出
        let new_b = index_b.wrapping_add(db);
        if new_a < heights.len() && new_b < heights[0].len() {
            if heights[new_a][new_b] >= heights[index_a][index_b] {
                to_ocean[new_a][new_b] = true;
                Self::helper(heights, visited_ocean, to_ocean, new_a, new_b)
            }
        }
    }
}