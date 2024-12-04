#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;

class Solution {
public:
    int lengthOfLongestSubstringKDistinct(string s, int k) {
        if (k == 0 || s.empty()) return 0; // 特殊情况处理
        unordered_map<char, int> map;     // 用于记录每个字符的出现次数
        int left = 0, maxLength = 0;

        for (int right = 0; right < s.length(); ++right) {
            map[s[right]]++; // 扩展窗口，增加字符计数

            // 如果当前窗口的不同字符数超过 k，收缩窗口
            while (map.size() > k) {
                map[s[left]]--;
                if (map[s[left]] == 0) {
                    map.erase(s[left]); // 删除频率为 0 的键
                }
                left++; // 移动窗口的左边界
            }

            // 更新最大长度
            maxLength = max(maxLength, right - left + 1);
        }

        return maxLength;
    }
};

// 测试代码
int main() {
    Solution solution;

    // 测试用例
    string s1 = "eceba";
    int k1 = 2;
    cout << "Test 1: " << solution.lengthOfLongestSubstringKDistinct(s1, k1) << endl; // 应输出 3

    string s2 = "aa";
    int k2 = 1;
    cout << "Test 2: " << solution.lengthOfLongestSubstringKDistinct(s2, k2) << endl; // 应输出 2

    string s3 = "a";
    int k3 = 2;
    cout << "Test 3: " << solution.lengthOfLongestSubstringKDistinct(s3, k3) << endl; // 应输出 1

    return 0;
}
