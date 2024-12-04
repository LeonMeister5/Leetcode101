/*
给你一个字符串 s，最多 可以从中删除一个字符。

请你判断 s 是否能成为回文字符串：如果能，返回 true ；否则，返回 false 。

 

示例 1：

输入：s = "aba"
输出：true
示例 2：

输入：s = "abca"
输出：true
解释：你可以删除字符 'c' 。
示例 3：

输入：s = "abc"
输出：false
 

提示：

1 <= s.length <= 105
s 由小写英文字母组成
*/

class Solution {
public:
    bool helper(string s, bool one_life){
        int size = s.length();
        if (size < 2){
            return true;
        }
        int left = 0; 
        int right = size - 1;
        while (left < right){
            if (s[left] == s[right]){
                left++;
                right--;
            }
            else if (one_life){
                std::string copya = s;
                copya.erase(left, 1);
                std::string copyb = s;
                copyb.erase(right, 1);
                return helper(copya, false) || helper(copyb, false);
            }
            else {
                return false;
            }
        }
        return true;
    }

    bool validPalindrome(string s) {
        return helper(s, true);
    }
};
