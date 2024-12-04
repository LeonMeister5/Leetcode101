/*
给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。

算法的时间复杂度应该为 O(log (m+n)) 。

 

示例 1：

输入：nums1 = [1,3], nums2 = [2]
输出：2.00000
解释：合并数组 = [1,2,3] ，中位数 2
示例 2：

输入：nums1 = [1,2], nums2 = [3,4]
输出：2.50000
解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 

 

提示：

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-106 <= nums1[i], nums2[i] <= 106
*/


class Solution {
public: // 我不会 GPT写的
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        if (nums1.size() > nums2.size()) {
            return findMedianSortedArrays(nums2, nums1);
        }

        int m = nums1.size();
        int n = nums2.size();
        int left = 0, right = m;
        int halfLen = (m + n + 1) / 2; // Position to split the arrays for median

        while (left <= right) {
            int i = (left + right) / 2;
            int j = halfLen - i;

            int nums1Left = (i == 0) ? INT_MIN : nums1[i - 1];
            int nums1Right = (i == m) ? INT_MAX : nums1[i];
            int nums2Left = (j == 0) ? INT_MIN : nums2[j - 1];
            int nums2Right = (j == n) ? INT_MAX : nums2[j];

            if (nums1Left <= nums2Right && nums2Left <= nums1Right) {
                // Found the correct partition
                if ((m + n) % 2 == 0) {
                    return (max(nums1Left, nums2Left) + min(nums1Right, nums2Right)) / 2.0;
                } else {
                    return max(nums1Left, nums2Left);
                }
            } else if (nums1Left > nums2Right) {
                // Adjust binary search range
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        throw invalid_argument("Input arrays are not sorted");
    }
};
