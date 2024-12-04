/*
给你一个字符串 s 和一个字符串数组 dictionary ，找出并返回 dictionary 中最长的字符串，该字符串可以通过删除 s 中的某些字符得到。

如果答案不止一个，返回长度最长且字母序最小的字符串。如果答案不存在，则返回空字符串。

 

示例 1：

输入：s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
输出："apple"
示例 2：

输入：s = "abpcplea", dictionary = ["a","b","c"]
输出："a"
 

提示：

1 <= s.length <= 1000
1 <= dictionary.length <= 1000
1 <= dictionary[i].length <= 1000
s 和 dictionary[i] 仅由小写英文字母组成
*/

class Solution {
public:
    string findLongestWord(string s, vector<string>& dictionary) {
        if (dictionary.size() == 0){
            return "";
        }
        if (s.length() == 0){
            return "";
        }
        std::sort(dictionary.begin(), dictionary.end(), [](const std::string& a, const std::string& b) {
            if (a.length() != b.length()) {
                return a.length() > b.length(); // 长度不同，长度大的在前
            } 
            else {
                return a < b; // 长度相同，按字典序（较小的在前）
            }
        });
        for(int i = 0; i < dictionary.size(); ++i){
            int s_id = 0;
            int dic_id = 0;
            while (dic_id < dictionary[i].length()){
                if (s[s_id] == dictionary[i][dic_id]){
                    s_id++;
                    dic_id++;
                }
                else {
                    s_id++;
                }
                if (s_id >= s.length()){
                    break;
                }
            }
            if (dic_id >= dictionary[i].length()){
                return dictionary[i];
            }
        }
        return "";
    }
};
