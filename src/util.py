import random

waifu_count = 9999  # actual count is waifu_count + 1 but this variable is needed for checks
waifu_len = len(str(waifu_count))


# Checks if ID is an integer AND in the bounds
def validate_id(waifu_id):
    try:
        return 0 <= int(waifu_id) <= waifu_count
    except ValueError:
        return False


# Returns a random waifu image by id and URL
def get_image_id():
    return 9876
    # return random.randint(0, waifu_count)
