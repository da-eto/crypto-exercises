#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <openssl/sha.h>

#define CHUNK_SIZE 1024

int main(int argc, char *argv[]) {
    if (argc != 2) {
        std::cout << "Use filename as single argument" << std::endl;

        return -1;
    }

    std::ifstream input(argv[1], std::ios::ate | std::ios::binary);

    if (!input.is_open()) {
        std::cout << "Can't open input file";

        return -2;
    }

    auto filesize = input.tellg();

    std::array<unsigned char, CHUNK_SIZE + SHA256_DIGEST_LENGTH> buffer{};
    std::array<unsigned char, SHA256_DIGEST_LENGTH> digest{};
    size_t digestLength = 0;

    for (auto pos = (filesize / CHUNK_SIZE) * CHUNK_SIZE; pos >= 0; pos -= CHUNK_SIZE) {
        input.clear();
        input.seekg(pos);
        input.read((char *) buffer.data(), CHUNK_SIZE);
        auto len = input.gcount();
        std::copy(digest.begin(), digest.begin() + digestLength, buffer.begin() + len);
        SHA256(buffer.data(), len + digestLength, digest.data());
        digestLength = SHA256_DIGEST_LENGTH;
    }

    input.close();
    std::cout << std::hex << std::setw(2) << std::setfill('0');

    for (auto it = digest.begin(); it < digest.end(); ++it) {
        std::cout << (int) *it;
    }

    std::cout << std::endl;

    return 0;
}
