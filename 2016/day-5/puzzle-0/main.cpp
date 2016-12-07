#include <iostream>
#include <cstring>
#include <vector>
#include <atomic>
#include <algorithm>
#include <iterator>
#include <numeric>
#include <future>

#include <openssl/md5.h>

#include "benchmark.hpp"
#include "locale.hpp"

// Problem variables
//   8 char password
constexpr auto password_len = 8u;
//   We only need the first 3 bytes of the digest
using Digest = unsigned char[3];
//   First 2 bytes must be 0 and 3rd byte must be <= 0x0f
const auto is_valid_digest = [](const Digest digest) {
    return (
        digest[0] == 0 &&
        digest[1] == 0 &&
        digest[2] <= 0x0f
    );
};
//   Last 4 bits of the 3rd byte as a hexadecimal character
const auto password_char_from_digest = [](const Digest digest) {
    return static_cast<char>(
        digest[2] < 0xa ?
        digest[2] + '0' :     // It's a digit
        digest[2] - 0xa + 'a' // It's a letter [a-f]
    );
};

// Data types
struct Match {
    char     character;
    unsigned iteration;
};
using Matches = std::vector<Match>;

// Thread synchronisation variables
//   Will keep track of the total number of matches found
std::atomic<unsigned> match_count;
//   Will keep track of the iteration number of the last match found
//   When the number of matches reach `password_len`, all the threads will have
//   to continue their iteration in order to make sure we did not skip a match
std::atomic<unsigned> highest_iteration;

// Writes `n` as a string in `buff`
// Much better than sprintf performance wise
// (most likely because of its reduced scope)
static int intcpy(char *buff, unsigned n) {
    auto s = 1,
         p = 1;

    while (n > p * 10) {
        ++s;
        p *= 10;
    }

    for (auto i = 0; i < s; ++i) {
        buff[s - i - 1] = n % 10 + '0';
        n /= 10;
    }

    buff[s] = 0;
    return s;
}

// Single unit of computation
// Will iterate starting from `start` and by steps of `step`
static auto step_compute(const char *door_id, unsigned start, unsigned step) {
    Matches matches;
    MD5_CTX context;
    Digest digest;
    char to_hash[32]; // Should be enough for small-ish lengths of door_id

    strcpy(to_hash, door_id);
    matches.reserve(password_len);

    const auto door_id_len = strlen(door_id);
    auto iter = start;

    while (match_count < password_len || iter < highest_iteration) {
        auto written_len = intcpy(to_hash + door_id_len, iter);

        MD5_Init(&context);
        MD5_Update(&context, to_hash, door_id_len + written_len);
        MD5_Final(digest, &context);

        if (is_valid_digest(digest)) {
            highest_iteration = std::max(highest_iteration.load(), iter);
            ++match_count;
            matches.emplace_back(Match {
                password_char_from_digest(digest),
                iter
            });
        }

        iter += step;
    }

    return matches;
}

static auto day_5_0(const char *door_id, unsigned th_count) {
    std::vector<std::thread> threads;
    std::vector<std::future<Matches>> futures;
    threads.reserve(th_count);
    futures.reserve(th_count);

    for (int i = 0; i < th_count; ++i) {
        auto compute = std::bind(step_compute, door_id, i, th_count);
        std::packaged_task<Matches()> task(compute);
        auto result = task.get_future();

        threads.emplace_back(std::move(task));
        futures.emplace_back(std::move(result));
    }

    for (auto & th: threads)
        th.join();

    auto accum_matches = [](auto & accum, auto & future) {
        auto matches = future.get();

        std::copy(
            matches.begin(), matches.end(),
            std::back_inserter(accum)
        );

        return accum;
    };

    auto all_matches = std::accumulate(
        futures.begin(), futures.end(),
        Matches { },
        accum_matches
    );

    std::sort(
        all_matches.begin(), all_matches.end(),
        [](auto m1, auto m2) { return m1.iteration < m2.iteration; }
    );

    Matches password_matches {
        all_matches.begin(),
        all_matches.begin() + password_len
    };

    std::cout << "Password: ";

    std::transform(
        password_matches.begin(), password_matches.end(),
        std::ostream_iterator<char>(std::cout),
        std::mem_fn(&Match::character)
    );

    std::cout << " (" << password_matches.back().iteration << " iterations)\n";
}

int main(int argc, char **argv) {
    if (argc < 2)
        return -1;

    auto door_id = argv[1];
    auto th_count = argc < 3 ? 1 : std::max(1, std::min(atoi(argv[2]), 64));

    std::cout.imbue(locale::group_numbers_locale);
    std::cout << "Computing hashes on " << th_count << " thread(s)\n";

    Benchmark b { "Elapsed" };

    day_5_0(door_id, th_count);

    return 0;
}
