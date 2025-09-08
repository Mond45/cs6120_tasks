#include <iostream>
#include <vector>

using namespace std;

int lengthOfLIS(vector<int> &nums) {
  const int N = nums.size();
  vector<int> dp(N, 1);
  int ans = 1;
  for (int i = 1; i < N; ++i) {
    for (int j = 0; j < i; ++j) {
      if (nums[j] < nums[i]) {
        dp[i] = max(dp[i], dp[j] + 1);
        ans = max(ans, dp[i]);
      }
    }
  }
  return ans;
}

int main() {
  int N = 8;
  vector<int> arr;
  while (N--) {
    int n;
    cin >> n;
    arr.push_back(n);
  }
  cout << lengthOfLIS(arr);
  return 0;
}
