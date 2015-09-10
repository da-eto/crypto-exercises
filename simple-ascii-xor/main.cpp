#include <iostream>
#include <sstream>
#include <vector>

typedef std::vector<uint8_t> uint8_vec;

uint8_vec hex_to_codes(std::string const &s) {
    uint8_vec codes;
    std::stringstream ss;
    ss << std::hex;

    if (s.length() % 2 == 0) {
        for (ulong i = 0; i < s.length(); i += 2) {
            int c = 0;
            ss << s.substr(i, 2) << ' ';
            ss >> c;
            codes.push_back((uint8_t) c);
        }
    }

    return codes;
}

std::string codes_to_hex(uint8_vec const &v) {
    std::stringstream ss;
    ss << std::hex;

    for (uint8_vec::const_iterator it = v.begin(); it != v.end(); ++it) {
        ss << (int) *it;
    }

    return ss.str();
}

uint8_vec xor_vec(uint8_vec const &a, uint8_vec const &b) {
    uint8_vec v;
    v.reserve(std::min(a.size(), b.size()));

    for (uint8_vec::const_iterator i = a.begin(), j = b.begin(); i != a.end() && j != b.end(); ++i, ++j) {
        v.push_back(*i ^ *j);
    }

    return v;
}

int main() {
    std::string iv, c, m, fake;

    std::getline(std::cin, iv);
    std::getline(std::cin, c);
    std::getline(std::cin, m);
    std::getline(std::cin, fake);

    uint8_vec iv_codes = hex_to_codes(iv);
    uint8_vec c_codes = hex_to_codes(c);

    for (auto it = iv_codes.begin(); it < iv_codes.end(); ++it) {
        std::cout << (int) *it << " ";
    }

    std::cout << std::endl;

    std::cout << codes_to_hex(c_codes);

    return 0;
}
