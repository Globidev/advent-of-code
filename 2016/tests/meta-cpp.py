import sys
import docker

day_count = 25
part_count = 2

irange = lambda end: range(1, end + 1)

def main():
    if len(sys.argv) < 3:
        return -1

    print('Running test suite: meta-cpp...')

    src_dir, lib_dir = sys.argv[1:]

    client = docker.from_env()

    command = lambda file_name: [
        'bash',
        '-c',
        'g++ -std=c++1z -O3 -I $BOOST_INCLUDE_DIR -I /common {}'.format(file_name)
    ]

    volumes = {
        src_dir: dict(bind='/srcs'),
        lib_dir: dict(bind='/common'),
    }

    for day in irange(day_count):
        for part in irange(part_count):
            file_name = 'day_{}-{}.cpp'.format(day, part)

            print('Day {}-{}: '.format(day, part), end='')
            sys.stdout.flush()

            try:
                client.containers.run(
                    image = 'my-cpp:7.0-snapshot',
                    command = command(file_name),
                    working_dir = '/srcs',
                    volumes = volumes,
                    remove = True
                )
                print('✔')
            except Exception as e:
                print('❌\n{}'.format(e.stderr.decode('utf-8')))


if __name__ == '__main__':
    main()
