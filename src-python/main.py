import json


def main():
    data = {
        "name": "John Doe",
        "age": 30,
        "city": "New York",
        "is_student": False,
        "lucky_numbers": [10 * n for n in range(10)],
    }

    # serialize and send to stdout
    print(json.dumps(data))


if __name__ == "__main__":
    main()
