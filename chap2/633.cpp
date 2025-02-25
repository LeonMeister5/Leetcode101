/*
给定一个非负整数 c ，你要判断是否存在两个整数 a 和 b，使得 a2 + b2 = c 。

 

示例 1：

输入：c = 5
输出：true
解释：1 * 1 + 2 * 2 = 5
示例 2：

输入：c = 3
输出：false
 

提示：

0 <= c <= 231 - 1
*/

class Solution {
public:
    bool judgeSquareSum(int c) {
        int a = 0;
        int b = static_cast<int>(std::sqrt(c));;
        while (a <= b){
            long long sum = static_cast<long long>(a) * a + static_cast<long long>(b) * b; 
            if (sum == c){
                return true;
            }
            else if (sum < c){
                a++;
            }
            else {
                b--;
            }
        }
        return false;
    }
};
