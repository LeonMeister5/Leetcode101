/*
给定一个字符串 s ，根据字符出现的 频率 对其进行 降序排序 。一个字符出现的 频率 是它出现在字符串中的次数。

返回 已排序的字符串 。如果有多个答案，返回其中任何一个。

 

示例 1:

输入: s = "tree"
输出: "eert"
解释: 'e'出现两次，'r'和't'都只出现一次。
因此'e'必须出现在'r'和't'之前。此外，"eetr"也是一个有效的答案。
示例 2:

输入: s = "cccaaa"
输出: "cccaaa"
解释: 'c'和'a'都出现三次。此外，"aaaccc"也是有效的答案。
注意"cacaca"是不正确的，因为相同的字母必须放在一起。
示例 3:

输入: s = "Aabb"
输出: "bbAa"
解释: 此外，"bbaA"也是一个有效的答案，但"Aabb"是不正确的。
注意'A'和'a'被认为是两种不同的字符。
 

提示:

1 <= s.length <= 5 * 105
s 由大小写英文字母和数字组成
*/



class Solution {
public:
    string frequencySort(string s) {
        // 用数组统计字符频率，假设输入字符在 ASCII 范围内
        std::array<int, 128> frequency = {0};
        for (char c : s) {
            frequency[c]++;
        }

        // 将非零频率的字符及其次数存入 vector
        std::vector<std::pair<char, int>> frequencyList;
        for (int i = 0; i < 128; ++i) {
            if (frequency[i] > 0) {
                frequencyList.emplace_back(i, frequency[i]);
            }
        }

        // 按出现次数从大到小排序
        std::sort(frequencyList.begin(), frequencyList.end(), [](const std::pair<char, int>& a, const std::pair<char, int>& b) {
            return a.second > b.second; // 按值降序排列
        });

        // 根据排序结果生成最终字符串
        std::string ans;
        for (const auto& [ch, freq] : frequencyList) {
            ans.append(freq, ch);
        }

        return ans;
    }
};
