#include <iostream>
#include <vector>
#include <list>
#include <iterator>
#include <algorithm>
#include <cstdint>

#define RANGE(l) std::begin(l), std::end(l)
#define foreach(it, l) for(auto it=std::begin(l); it != std::end(l); it++)

using namespace std;

auto solve(int B, int M) -> std::string {
    int64_t x = 0x01L << (B-2);

    if (M > x) {
	return "IMPOSSIBLE";
    }

    int arr[B][B];
    for(int i=0; i<B; i++) {
	for(int j=0; j<B; j++) {
	    arr[i][j] = (i>=j)?0:1;
	}
    }
    int z = 0;
    for(int i=0; i<B; i++) {
	int k, j;
	for(k=(0x01 << (B-2-i)), j=i+1; j<B; k/=2, j++)
	{
	    if(k == 0) {
		if(z + 1 < M) {
		    arr[i][j] = 0;
		    z += 1;
		}
	    } else {
		if(x + k < M) {
		    arr[i][j] = 0;
		    z += k;
		}
	    }
	}
    }

    std::string res = "POSSIBLE";
    for(int i=0; i<B; i++) {
	res += "\n";
	for(int j=0; j<B; j++) {
	    res += arr[i][j] + '0';
	}
    }
    return res;
}

auto main() -> int {
    int T;
    std::cin >> T;

    for(int i=1; i<= T; i++) {
	int B, M;
	std::cin >> B >> M;
        std::cout << "Case #" << i << ": " << solve(B, M) << std::endl;
    }

    return 0;
}
