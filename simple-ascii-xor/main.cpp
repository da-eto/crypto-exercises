#include <iostream>
#include <sstream>
#include <vector>

std::vector<uint8_t> to_codes(std::string const &s) {
    std::vector<uint8_t> codes;
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

std::string from_codes(std::vector<uint8_t> const &v) {
    std::string s;

    std::stringstream ss;
    ss << std::hex;

    for (std::vector<uint8_t>::const_iterator it = v.begin(); it != v.end(); ++it) {
        ss << (int) *it;
    }

    return ss.str();
}

int main() {
    std::string iv, c, m, fake;

    std::getline(std::cin, iv);
    std::getline(std::cin, c);
    std::getline(std::cin, m);
    std::getline(std::cin, fake);

    std::vector<uint8_t> iv_codes = to_codes(iv);
    std::vector<uint8_t> c_codes = to_codes(c);

    for (auto it = iv_codes.begin(); it < iv_codes.end(); ++it) {
        std::cout << (int) *it << " ";
    }

    std::cout << std::endl;

    std::cout << from_codes(c_codes);

    return 0;
}
