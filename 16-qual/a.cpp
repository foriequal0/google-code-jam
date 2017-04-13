#include <iostream>
#include <stdint.h>

using namespace std;

void mark_digits(uint16_t &bitmask, int x) {
    if(x == 0) {
        bitmask &= ~0x01;
        return;
    }

    while(x != 0) {
        int d = x % 10;
        x /= 10;
        bitmask &= ~(0x01 << d);
    }
}

void solve(int t, int N) {
    uint16_t bitmask = 0x3ff;

    if(N == 0) {
        std::cout << "Case #" << t << ": INSOMNIA" << std::endl;
    } else {
        int x = 0;
        while(bitmask != 0) {
            x += N;
            mark_digits(bitmask, x);
        }
        std::cout << "Case #" << t << ": " << x << std::endl;
    }
}

int main() {
    int T;
    std::cin >> T;

    for(int i=1; i<=T; i++) {
        int N;
        std::cin >> N;

        solve(i, N);
    }
    return 0;
}
