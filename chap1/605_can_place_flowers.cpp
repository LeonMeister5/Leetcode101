/*
假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。

给你一个整数数组 flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n ，能否在不
打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false 。
 

示例 1：

输入：flowerbed = [1,0,0,0,1], n = 1
输出：true
示例 2：

输入：flowerbed = [1,0,0,0,1], n = 2
输出：false
*/

class Solution {
public:
    bool canPlaceFlowers(vector<int>& flowerbed, int n) {
        if (flowerbed.size() == 0){
            if (n == 0){
                return true;
            }
            return false;
        }
        if (flowerbed.size() == 1){
            if (n == 1 && flowerbed[0] == 0){
                return true;
            }
            else if (n == 0){
                return true;
            }
            return false;
        }

        int capacity = 0;
        if (flowerbed[0] == 0 && flowerbed[1] == 0){
            capacity++;
            flowerbed[0] = 1;
        }

        for (int j = 1; j < flowerbed.size() - 1; ++j){
            if (flowerbed[j - 1] == 0 && flowerbed[j] == 0 && flowerbed[j + 1] == 0){
                capacity++;
                flowerbed[j] = 1;
            }
        }

        if (flowerbed[flowerbed.size() - 2] == 0 && flowerbed[flowerbed.size() - 1] == 0){
            capacity++;
            flowerbed[flowerbed.size() - 1] = 1;
        }

        if (capacity < n){
            return false;
        }
        return true;
    }
};
