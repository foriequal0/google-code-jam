#include <iostream>
#include <vector>
#include <list>
#include <iterator>
#include <algorithm>

#define RANGE(l) std::begin(l), std::end(l)
#define foreach(it, l) for(auto it=std::begin(l); it != std::end(l); it++)

using namespace std;

struct party {
    char name;
    int count;
    party(char name, int count) :name(name), count(count) {}
};

auto solve(int sum, std::vector<party> v) -> std::string {
    std::string solution = "";
    while(sum > 0) {
	std::sort(RANGE(v), [](const party &x, const party &y)
		  { return x.count > y.count; });
    A:
	{ // [0] - 1
	    if (v[0].count < 1) goto B;

	    auto nsum = sum - 1;
	    if(nsum < 0) goto B;

	    auto second = v[1].count;
	    if (second * 2 > nsum) goto B;

	    solution += v[0].name;
	    v[0].count -= 1;
	    sum = nsum;

	    goto CONT;
	} 
    B:
	{ // [0] - 1, [1] - 1
	    if (v[0].count < 1 || v[1].count < 1) goto C;

	    auto nsum = sum - 2;
	    if(nsum < 0) goto C;
	    
	    auto second = v[1].count - 1;
	    auto third = (v.size() > 2) ? v[2].count : 0;
	    if(second * 2 > nsum || third * 2 > nsum) goto C;

	    solution += v[0].name;
	    solution += v[1].name;
	    v[0].count -= 1;
	    v[1].count -= 1;
	    sum = nsum;
	    
	    goto CONT;
	}
    C:
	{
	    if (v[0].count < 2) goto CONT;

	    auto nsum = sum - 2;
	    if(nsum < 0) goto CONT;

	    auto second = v[1].count;
	    if(second * 2 > nsum) goto CONT;

	    solution += v[0].name;
	    solution += v[0].name;
	    v[0].count -= 2;

	    sum = nsum;
	    
	    goto CONT;
	}
	
    CONT:
	solution += " ";
    }

    return solution;
}

auto main() -> int {
    int T;
    std::cin >> T;

    for(int i=1; i<= T; i++) {
	int sum = 0;
	std::vector<party> v;
	
	int N;
	std::cin >> N;
	for(int j=0; j<N; j++) {
	    int count;
	    std::cin >> count;
	    
	    v.emplace_back(j+'A', count);
	    sum += count;
	}
        std::cout << "Case #" << i << ": " << solve(sum, v) << std::endl;
    }

    return 0;
}
