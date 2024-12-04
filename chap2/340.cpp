/*
340. Longest Substring with At Most K Distinct Characters
Given a string, find the length of the longest substring T that contains at most k distinct characters.

Example 1:

Input: s = "eceba", k = 2
Output: 3
Explanation: T is "ece" which its length is 3.
Example 2:

Input: s = "aa", k = 1
Output: 2
Explanation: T is "aa" which its length is 2.
*/

// No online judge，随便写写

class Solution {
public:
    int lengthOfLongestSubstringKDistinct(string s, int k) {
        int maxlength = s.length();
        if (k == 0){
            return 0;
        }
        else if (s.length() == 0){
            return 0;
        }
        unordered_map<char, int> map;
        for (int i = 0; i < s.length(); ++i){
            map[s[i]]++;
        }
        int left = 0;
        int right = maxlength - 1;
        while (map.size() > k){ // 这里写的是错的，应该删除剩1个，而不是删到0个。
                                // GPT:窗口只从左边开始删。通过检查每一个right的maxlength，来完成整个数组的maxlength。
            if (map[s[left]] == 1){
                map.erase(s[left]);
                left++;
                continue;
            }
            else if (map[s[right]] == 1){
                map.erase(s[right]);
                right--;
                continue;
            }
            else {
                map[s[left]]--;
                left++;
                continue;
            }
        }
        return right - left + 1;
    }
};
