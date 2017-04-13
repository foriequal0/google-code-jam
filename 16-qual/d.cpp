#include <iostream>
#include <cmath>
#include <stdint.h>

void solve(int K, int C, int S) {
    int s = K - C + 1;
    if (S < s) {
	std::cout << "IMPOSSIBLE";
    } else {
	uint64_t a = 0;
	int b = 0;
	int s = K;

	for(int d = 0; d < C; d++) {
	    s = K - d;
	    a = (a + b) * K;
	    b = d;
	    //std::cout << d << ": " << s << " " << a << ", " << b << std::endl;
	    if(s == 1)
		break;
	}
	
	auto offset = a+b;
	for(auto i= offset + 1; i<= offset + s; i++) {
	    std::cout << i << " ";
	}
    }
}

int main(){
    int T;
    std::cin >> T;

    for(int t = 1; t <= T; t++) {
	int K, C, S;
	std::cin >> K >> C >> S;
	std::cout << "Case #" << t << ": ";
	solve(K, C, S);
	std::cout << std::endl;
    }
}
