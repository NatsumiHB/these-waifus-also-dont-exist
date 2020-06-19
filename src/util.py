import random

waifu_count = 9999  # actual count is waifu_count + 1 but this variable is needed for checks


# Checks if ID is an integer AND in the bounds
def validate_id(waifu_id):
    try:
        waifu_id = int(waifu_id)
    except:
        return False

    if waifu_id > waifu_count or waifu_id < 0:
        return False

    return True


# Returns a random waifu image by id and URL
def get_image():
    return random.randint(0, waifu_count)
