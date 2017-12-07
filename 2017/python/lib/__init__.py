from os.path import split, join, realpath, splitext

def readinput(src_file, strip=True):
    full_path = realpath(src_file)
    src_dir, file_name = split(full_path)
    day, _ = splitext(file_name)

    with open(join(src_dir, '../inputs', day)) as f:
        data = f.read()
        return data.strip() if strip else data
