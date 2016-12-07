#include <chrono>

struct Benchmark {

    using Clock = std::chrono::high_resolution_clock;

    Benchmark(const std::string & description):
        description_ { description }, t0_ { Clock::now() }
    { }

    ~Benchmark() {
        using std::chrono::duration_cast;
        using nanoseconds = std::chrono::nanoseconds;

        auto t1 = Clock::now();
        auto duration = duration_cast<nanoseconds>(t1 - t0_);

        std::cout << description_ << ": "
                  << duration.count() << " ns" << std::endl;
    }

private:

    std::string description_;
    Clock::time_point t0_;

};
