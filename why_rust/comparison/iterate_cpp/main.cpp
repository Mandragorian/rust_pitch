#include <iostream>
#include <vector>

using namespace std;

int main()
{
	vector<int> lst = { 1, 2, 3, 4};
	for (auto &elem: lst) {
		cout << elem << "\n";
		lst.push_back(elem);
	}
	return 0;
}
