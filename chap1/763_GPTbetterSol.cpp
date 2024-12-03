class Solution {
public:
    vector<int> partitionLabels(string s) {
        vector<int> ans;

        // Step 1: 记录每个字母的首尾位置
        vector<int> last_pos(26, -1); // 用于记录每个字母的最后出现位置
        for (int i = 0; i < s.length(); ++i) {
            last_pos[s[i] - 'a'] = i;
        }

        // Step 2: 遍历字符串划分区间
        int start = 0, end = 0; // 当前片段的开始和结束位置
        for (int i = 0; i < s.length(); ++i) {
            end = max(end, last_pos[s[i] - 'a']); // 更新当前片段的结束位置
            if (i == end) { // 到达当前片段的结束位置
                ans.emplace_back(end - start + 1); // 记录片段长度
                start = i + 1; // 更新起始位置为下一个片段
            }
        }

        return ans;
    }
}