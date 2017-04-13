#include <iostream>
#include <stdint.h>
#include <vector>
#include <iterator>
#include <algorithm>
#include <limits>

const std::vector<int> known_prime_numbers = {
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997
};
class Integer;
std::ostream &operator <<(std::ostream &os, const Integer &x);
class Integer {
public:
    Integer() = default;

    Integer(uint64_t l) {
	i[0] = l;
    }

    Integer(uint64_t h, uint64_t l) {
	i[0] = l;
	i[1] = h;
    }

    Integer(const Integer &other) = default;
    Integer(Integer &&other) = default;
    virtual ~Integer() = default;
    Integer& operator=(const Integer &other) = default;
    Integer& operator=(Integer &&other) = default;

    int operator %(int x) const {
	if(this->i[1] == 0) {
	    return this->i[0] % x;
	} else {
	    Integer rem = *this;
	    Integer X(x);
	    int i;
	    for(i=0; i<128; i++) {
		auto newX = X.lsh1();
		if (rem.compare_to(newX) <= 0)
		    break;
		X = newX;
	    }

	    for(; i>=0; i--) {
		if(rem.compare_to(X) >=0) {
		    rem = rem - X;
		    
		}
		X = X.rsh1();
		if(rem.i[0] == 0 && rem.i[1] == 0)
		    return 0;
	    }
	    return rem.i[0];
	}
    }

    int compare_to(const Integer &other) const {
	if (this->i[1] == other.i[1]) {
	    if(this->i[0] < other.i[0]) {
		return -1;
	    } else if(this->i[0] > other.i[0]) {
		return 1;
	    } else {
		return 0;
	    }
	}
	else if (this->i[1] < other.i[1]) {
	    return -1;
	} else {
	    return 1;
	} 
    }

    Integer operator+(int x) const {
	auto headroom = std::numeric_limits<uint64_t>::max() - i[0];
	if(x > headroom) {
	    return Integer(i[1] + 1, x - headroom);
	} else {
	    return Integer(i[1], i[0] + x);
	}
    }

    Integer operator+(const Integer &other) const {
	auto headroom = std::numeric_limits<uint64_t>::max() - i[0];
	if(other.i[0] > headroom) {
	    return Integer(i[1] + other.i[1] + 1, i[0] + other.i[0]);
	} else {
	    return Integer(i[1] + other.i[1], i[0] + other.i[0]);
	}
    }

    Integer operator-(const Integer &other) const {
	if (this->i[0] < other.i[0]) {
	    return Integer(i[1] - other.i[1] - 1, std::numeric_limits<uint64_t>::max() - other.i[0] + this->i[0] + 1);
	} else {
	    return Integer(i[1] - other.i[1], this->i[0] - other.i[0]);
	}
    }

    Integer lsh1() const {
	return Integer((i[1] << 1) + (i[0]>>63), i[0] << 1);
    }

    Integer rsh1() const {
	return Integer((i[1] >> 1), (i[1] << 63) + (i[0] >> 1));
    }

    static Integer mult64(uint64_t u, uint64_t v) {
	uint64_t
	    ul = u & 0xffffffff,
	    uh = u >> 32,
	    vl = v & 0xffffffff,
	    vh = v >> 32;

	uint64_t
	    a0 = ul * vl,
	    a1 = ul * vh,
	    a2 = uh * vl,
	    a3 = uh * vh;

	uint64_t
	    b0 = a0 & 0xffffffff,
	    b1 = (a0 >> 32) + (a1 & 0xffffffff) + (a2 & 0xffffffff),
	    b2 = (b1 >> 32) + (a1 >> 32) + (a2 >> 32) + (a3 & 0xffffffff),
	    b3 = (b2 >> 32) + (a3 >> 32);

	return Integer((b3 << 32) + (b2 & 0xffffffff),
		       (b1 << 32) + (b0 & 0xffffffff));
    }
    
    Integer operator*(int x) const {
	auto a = Integer::mult64(i[0], x);
	return Integer(a.i[1] + i[1] * x, a.i[0]);
    }
    
    static Integer interpret(uint32_t x, int radix) {
	Integer result;
	Integer floating { 1 };

	while(x != 0) {
	    int bit = x & 0x01;
	    if(bit)
		result = result + floating;
	    
	    floating = floating * radix;
	    x >>= 1;
	}

	return result;
    }
    
    uint64_t i[2] = {0, };
};

std::ostream &operator <<(std::ostream &os, const Integer &x) {
    char buf[32];
    sprintf(buf, "%lx%016lx", x.i[1], x.i[0]);
    os << buf;
    return os;
}

int divisor(Integer x) {
    auto begin = std::begin(known_prime_numbers);
    auto end = std::end(known_prime_numbers);
    for(auto iter = begin; iter != end; iter++) {
	auto d = *iter;
	if(x % d == 0) {
	    return d;
	}
    }
    return 0;
}

std::ostream &print_binary(std::ostream &os, uint32_t x, int width) {
     for(int shift = width-1; shift >= 0; shift--) {
	os << ((x >>shift) & 0x01);
    }
    return os;
}

void solve(int i, int N, int J) {
    uint64_t jc = (((uint64_t)0x01) << (N-1)) + 1;

    int count = 0;
    while(count < J && ((jc >> N) < (0x01))) {
	Integer jc_interprets[11];

	for(int radix=2; radix<=10; radix++) {
	    jc_interprets[radix] = Integer::interpret(jc, radix);
	}

	int divisors[11];
	int radix;
	for(radix=2; radix <= 10; radix++) {
	    auto d = divisor(jc_interprets[radix]);
	    if(d > 0) {
		divisors[radix] = d;
	    } else {
		break;
	    }
	}

	if(radix == 11) {
	    print_binary(std::cout, jc, N) << ' ';
	    for(int radix = 2; radix <= 10; radix++) {
		std::cout << divisors[radix];
		if(radix < 10) {
		    std::cout << ' ';
		} else {
		    std::cout << std::endl;
		}
	    }
	    count++;
	} 

	jc += 2;
    }
}

int main() {
    int T;
    std::cin >> T;
    
    for(int i=1; i<= T; i++) {
	int N, J;
	std::cin >> N >> J;
	std::cout << "Case #" << i << ": " << std::endl;
	solve(i, N, J);
    }
    
    return 0;
}
