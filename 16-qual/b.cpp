#include <iostream>
#include <iterator>
#include <stdint.h>

using namespace std;

ostream& print_case(ostream &os, int i) {
    return os << "Case #" << i << ": ";
}

int solve(const string &x) {
    if(x.length() == 0) {
	return 0;
    }
    
    string guarded = x + "+";

    int edge_count = 0;
    auto iter = begin(guarded);
    auto prev = *iter; iter++;
    do {
	auto curr = *iter; iter++;
	if(curr != prev) {
	    edge_count += 1;
	}
	prev = curr;
    } while(iter != end(guarded));

    return edge_count;
}

int main() {
    int T;
    cin >> T;

    for(int i=1; i<=T; i++) {
	string pancakes;
	cin >> pancakes;
	int solution = solve(pancakes);
	print_case(cout, i) << solution << endl;
    }
}
