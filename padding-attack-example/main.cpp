#include <iostream>
#include <istream>
#include <ostream>
#include <string>
#include <boost/asio.hpp>
#include <iomanip>

using boost::asio::ip::tcp;

typedef std::vector<uint8_t> uint8_vec;

uint8_vec hex_to_codes(const std::string &s) {
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

std::string codes_to_hex(const uint8_vec &v) {
    std::stringstream ss;
    ss << std::hex;

    for (uint8_vec::const_iterator it = v.begin(); it != v.end(); ++it) {
        ss << std::setw(2) << std::setfill('0') << (int) *it;
    }

    return ss.str();
}


class Oracle {
public:
    Oracle(boost::asio::io_service &service, const char *domain_name) : domain(domain_name), io_service(service) {
        // Get a list of endpoints corresponding to the server name.
        tcp::resolver resolver(io_service);
        tcp::resolver::query query(domain, "http");
        endpoint_iterator = resolver.resolve(query);
    }

    unsigned int query(const std::string &param) {
        // Try each endpoint until we successfully establish a connection.
        tcp::socket socket(io_service);
        boost::asio::connect(socket, endpoint_iterator);

        // Form the request. We specify the "Connection: close" header so that the
        // server will close the socket after transmitting the response. This will
        // allow us to treat all data up until the EOF as the content.
        boost::asio::streambuf request;
        std::ostream request_stream(&request);
        request_stream << "GET /po?er=" << param << " HTTP/1.0\r\n";
        request_stream << "Host: " << domain << "\r\n";
        request_stream << "Accept: */*\r\n";
        request_stream << "Connection: close\r\n\r\n";

        // Send the request.
        boost::asio::write(socket, request);

        // Read the response status line. The response streambuf will automatically
        // grow to accommodate the entire line. The growth may be limited by passing
        // a maximum size to the streambuf constructor.
        boost::asio::streambuf response;
        boost::asio::read_until(socket, response, "\r\n");

        // Get response status code.
        std::istream response_stream(&response);
        std::string http_version;
        response_stream >> http_version; // maybe garbage; skip it
        unsigned int status_code;
        response_stream >> status_code;

        return status_code;
    }

private:
    std::string domain;
    boost::asio::io_service &io_service;
    tcp::resolver::iterator endpoint_iterator;
};

class Solver {
public:
    Solver(Oracle *oracle) : oracle(oracle) { }

    void decipher_block(const uint8_t *iv, const uint8_t *cipher_text, uint8_t *message) {
        uint8_vec guess(0x30, 0);
//        std::copy(iv, iv + 0x10, guess.data() + 0x10);
        std::copy(cipher_text, cipher_text + 0x10, guess.data() + 0x20);

        std::cout << "# decipher block" << std::endl << std::endl;
        std::cout.flush();

        for (uint8_t pad = 0x01; pad <= 0x10; ++pad) {
            std::cout << "pad: " << (int) pad << std::endl;
            std::cout.flush();

            int pos = 0x10 - pad;

            for (int p = pos + 1; p < 0x10; ++p) {
                guess[p + 0x10] = iv[p] ^ message[p] ^ pad;
            }

            for (int g = 0x00; g < 0x100; ++g) {
                guess[pos + 0x10] = iv[pos] ^ (uint8_t) g ^ pad;
                std::string query = codes_to_hex(guess);
                unsigned int code = oracle->query(query);

                std::cout << g << ":" << code << "\t" << query << std::endl;
                std::cout.flush();

                if (code != 403) {
                    std::cout << "found: " << g << " (char: '" << (char) g << "')" << std::endl << std::endl;
                    std::cout.flush();

                    message[pos] = (uint8_t) g;
                    break;
                }
            }
        }
    }

private:
    Oracle *oracle;
};

int main(int argc, char *argv[]) {
    try {
        if (argc != 3) {
            std::cout << "Use arguments, Luke\n";

            return -1;
        }

        std::string cipher_hex(argv[2]);
        uint8_vec cipher = hex_to_codes(cipher_hex);
        uint8_vec message(cipher.size() - 0x10, 0);
        int block_count = message.size() / 0x10;

        boost::asio::io_service io_service;
        Solver *solver = new Solver(new Oracle(io_service, argv[1]));

        for (int block = block_count - 1; block >= 0; --block) {
            solver->decipher_block(cipher.data() + block * 0x10,
                                   cipher.data() + block * 0x10 + 0x10,
                                   message.data() + block * 0x10);
        }

        std::cout << "message hex: " << codes_to_hex(message) << std::endl;
        message[message.size() - message[message.size() - 1]] = 0;
        std::cout << "message: '" << (char *) message.data() << "'" << std::endl;
    } catch (std::exception &e) {
        std::cout << "Exception: " << e.what() << "\n";
    }

    return 0;
}