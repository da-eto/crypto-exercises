#include <iostream>
#include <sstream>
#include <vector>
#include <iomanip>

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
        ss << std::setw(2) << std::setfill('0') << (int) *it;
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
    std::string iv_str, message_str, fake_str;

    std::getline(std::cin, iv_str);
    std::getline(std::cin, message_str);
    std::getline(std::cin, fake_str);

    uint8_vec iv = hex_to_codes(iv_str);
    uint8_vec m(message_str.begin(), message_str.end());
    uint8_vec fake(fake_str.begin(), fake_str.end());

    std::cout << codes_to_hex(xor_vec(iv, xor_vec(m, fake))) << std::endl;

    return 0;
}
