import os

def get_version():
    return os.getenv("VERSIONEER_OVERRIDE", default="7.8123.3038639")
