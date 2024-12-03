/*
763. 划分字母区间
中等
相关标签
相关企业
提示
给你一个字符串 s 。我们要把这个字符串划分为尽可能多的片段，同一字母最多出现在一个片段中。

注意，划分结果需要满足：将所有划分结果按顺序连接，得到的字符串仍然是 s 。

返回一个表示每个字符串片段的长度的列表。

 

示例 1：
输入：s = "ababcbacadefegdehijhklij"
输出：[9,7,8]
解释：
划分结果为 "ababcbaca"、"defegde"、"hijhklij" 。
每个字母最多出现在一个片段中。
像 "ababcbacadefegde", "hijhklij" 这样的划分是错误的，因为划分的片段数较少。 
示例 2：

输入：s = "eccbbbbdec"
输出：[10]
 

提示：

1 <= s.length <= 500
s 仅由小写英文字母组成
*/

class Solution {
public:
    vector<int> partitionLabels(string s) {
        vector<int> ans;
        vector<vector<int>> first_and_last;
        vector<bool> exists(26, false);
        for (char c : s) {
            exists[c - 'a'] = true;
        }
        for (char i = 'a'; i <= 'z'; ++i) {
            if (exists[i - 'a']) {
                int first = -1, last = -1;
                for (int j = 0; j < s.length(); ++j) {
                    if (s[j] == i) {
                        if (first == -1) first = j;
                        last = j;
                    }
                }
                first_and_last.emplace_back(vector<int>{first, last});
            }
        }
        sort(first_and_last.begin(), first_and_last.end(), [](const vector<int>& a, const vector<int>& b) {
            return a[0] < b[0];
        });
        int current_start = 0;
        int current_end = first_and_last[0][1];
        int currentcharid = 0;
        int charid = 1;
        while (charid < first_and_last.size()){
            if (first_and_last[charid][0] > current_end){
                ans.emplace_back(first_and_last[charid][0] - current_start);
                current_start = first_and_last[charid][0];
                current_end = first_and_last[charid][1];
                charid++;
                continue;
            }
            if (first_and_last[charid][1] < current_end){
                charid++;
                continue;
            }
            else {
                current_end = first_and_last[charid][1];
                charid++;
                continue;
            }
        }
        ans.emplace_back(current_end - current_start + 1);
        return ans;
    }
};
